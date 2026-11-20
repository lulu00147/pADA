pub mod ast_interceptor;
pub mod csv_encoding;
pub mod extension_guard;
pub mod path_validator;
pub mod public_alias;
use crate::config::ServerConfig;
pub use ast_interceptor::AstInspector;
pub use csv_encoding::ensure_utf8_inplace;
pub use csv_encoding::is_valid_utf8;
pub use extension_guard::ExtensionGuard;
pub use path_validator::{resolve_registration_path, validate_path};
use regex::Regex;
use sqlparser::ast::{
    Expr, JoinConstraint, JoinOperator, ObjectName, Query, SetExpr, Statement, TableFactor,
    TableWithJoins, Visit, Visitor,
};
use sqlparser::parser::Parser;
use std::collections::HashMap;
use std::ops::ControlFlow;
use std::sync::LazyLock;
pub fn is_public_alias(name: &str) -> bool {
    public_alias::is_public_alias(name)
}
fn looks_like_numeric_table(sql: &str) -> bool {
    static NUM_TABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)\bFROM\s+"?(\d+)"?(?:\s+(FOR|AS|JOIN|WHERE|GROUP|ORDER|HAVING|LIMIT|UNION|EXCEPT|INTERSECT|OFFSET|ON|USING|;|$))?"#)
            .expect("static NUM_TABLE_RE is valid")
    });
    for caps in NUM_TABLE_RE.captures_iter(sql) {
        let after = caps.get(2).map(|m| m.as_str());
        if after.is_some_and(|w| w.eq_ignore_ascii_case("FOR")) {
            continue;
        }
        return true;
    }
    false
}
fn validate_public_object_name(name: &ObjectName) -> Result<(), String> {
    let parts: Vec<String> = name.0.iter().map(|part| part.value.clone()).collect();
    if public_alias::is_valid_public_object_name(&parts) {
        return Ok(());
    }
    match parts.as_slice() {
        [schema, object] if schema.eq_ignore_ascii_case("main") => Err(format!(
            "main.{object} is not a public data alias; use a vw_... view. execute_read_sql and pipeline kind=query can only read vw_ views. Scratchpad tm_* tables are write-only; never query table names."
        )),
        [object] => Err(format!(
            "{object} is not a public data alias; use a vw_... view. execute_read_sql and pipeline kind=query can only read vw_ views. Scratchpad tm_* tables are write-only; never query table names."
        )),
        _ => Err("Only public vw_ views may be queried on the read path. execute_read_sql and pipeline kind=query cannot read tm_* tables; query vw_... instead.".into()),
    }
}
fn is_direct_file_loader(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "READ_CSV"
            | "READ_CSV_AUTO"
            | "READ_JSON"
            | "READ_JSON_AUTO"
            | "READ_PARQUET"
            | "READ_EXCEL"
            | "READ_XLSX"
            | "ST_READ"
            | "ST_READTEXT"
            | "ST_READFILE"
            | "READFILE"
    )
}
fn direct_file_function_error() -> ControlFlow<String> {
    ControlFlow::Break("Direct file functions are not allowed; use a registered vw_ alias".into())
}
struct QueryScope {
    visible: Vec<String>,
}
struct PublicRelationVisitor {
    skip_target: Option<ObjectName>,
    query_scopes: Vec<QueryScope>,
    pending_query_scopes: HashMap<*const Query, Vec<String>>,
}
impl Visitor for PublicRelationVisitor {
    type Break = String;
    fn pre_visit_relation(&mut self, relation: &ObjectName) -> ControlFlow<Self::Break> {
        let cte_visible = self
            .query_scopes
            .last()
            .map(|scope| {
                relation.0.len() == 1
                    && scope
                        .visible
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(&relation.0[0].value))
            })
            .unwrap_or(false);
        if self.skip_target.as_ref() == Some(relation) || cte_visible {
            return ControlFlow::Continue(());
        }
        match validate_public_object_name(relation) {
            Ok(()) => ControlFlow::Continue(()),
            Err(error) => ControlFlow::Break(error),
        }
    }
    fn pre_visit_query(&mut self, query: &Query) -> ControlFlow<Self::Break> {
        let inherited = self
            .pending_query_scopes
            .remove(&(query as *const Query))
            .or_else(|| self.query_scopes.last().map(|scope| scope.visible.clone()))
            .unwrap_or_default();
        let cte_names: Vec<String> = query
            .with
            .as_ref()
            .map(|with| {
                with.cte_tables
                    .iter()
                    .map(|cte| cte.alias.name.value.clone())
                    .collect()
            })
            .unwrap_or_default();
        let mut prior = inherited.clone();
        if let Some(with) = &query.with {
            for cte in &with.cte_tables {
                self.pending_query_scopes
                    .insert(&*cte.query as *const Query, prior.clone());
                prior.push(cte.alias.name.value.clone());
            }
        }
        let mut visible = inherited;
        visible.extend(cte_names);
        self.query_scopes.push(QueryScope { visible });
        ControlFlow::Continue(())
    }
    fn post_visit_query(&mut self, _query: &Query) -> ControlFlow<Self::Break> {
        self.query_scopes.pop();
        ControlFlow::Continue(())
    }
    fn pre_visit_expr(&mut self, expr: &Expr) -> ControlFlow<Self::Break> {
        if let Expr::Function(function) = expr {
            if function.name.0.len() == 1 && is_direct_file_loader(&function.name.0[0].value) {
                return direct_file_function_error();
            }
        }
        ControlFlow::Continue(())
    }
    fn pre_visit_table_factor(&mut self, factor: &TableFactor) -> ControlFlow<Self::Break> {
        if let TableFactor::TableFunction { expr, .. } = factor {
            let function = expr.to_string();
            let name = function.split('(').next().unwrap_or(&function).trim();
            if is_direct_file_loader(name) {
                return direct_file_function_error();
            }
        }
        ControlFlow::Continue(())
    }
}
fn parse_public_statements(sql: &str) -> Result<Vec<Statement>, String> {
    Parser::parse_sql(&sqlparser::dialect::GenericDialect, sql)
        .map_err(|_| "SQL must be parser-recognized before relation validation".to_string())
}
pub fn validate_public_read_relations(sql: &str) -> Result<(), String> {
    for statement in &parse_public_statements(sql)? {
        let mut visitor = PublicRelationVisitor {
            skip_target: None,
            query_scopes: Vec::new(),
            pending_query_scopes: HashMap::new(),
        };
        match statement.visit(&mut visitor) {
            ControlFlow::Continue(()) => {}
            ControlFlow::Break(error) => return Err(error),
        }
    }
    Ok(())
}
pub fn validate_public_write_relations(sql: &str) -> Result<(), String> {
    for statement in &parse_public_statements(sql)? {
        let target = match statement {
            Statement::CreateTable { name, .. } => Some(name.clone()),
            Statement::Insert { table_name, .. } => Some(table_name.clone()),
            Statement::Update { table, .. } => write_target_name_from_table(table),
            Statement::Delete { from, .. } => write_target_name_from_delete(from),
            Statement::AlterTable {
                name, operations, ..
            } => {
                validate_alter_operations(operations)?;
                Some(name.clone())
            }
            Statement::Drop { names, .. } => {
                for name in names {
                    validate_write_target(name)?;
                }
                names.first().cloned()
            }
            _ => None,
        };
        if let Some(name) = &target {
            validate_write_target(name)?;
        }
        let mut visitor = PublicRelationVisitor {
            skip_target: target,
            query_scopes: Vec::new(),
            pending_query_scopes: HashMap::new(),
        };
        match statement.visit(&mut visitor) {
            ControlFlow::Continue(()) => {}
            ControlFlow::Break(error) => return Err(error),
        }
    }
    Ok(())
}
fn validate_write_target(name: &ObjectName) -> Result<(), String> {
    let parts: Vec<String> = name.0.iter().map(|part| part.value.clone()).collect();
    if public_alias::is_valid_write_target(&parts) {
        return Ok(());
    }
    match parts.as_slice() {
        [schema, _object] => Err(format!(
            "Writes to schema '{schema}' are not allowed; use unqualified tm_<16 hex> in ai_scratchpad"
        )),
        _ => Err("Write targets must be tm_<16 hex> (unqualified) or ai_scratchpad.tm_<16 hex>".into()),
    }
}
fn write_target_name_from_table(table: &TableWithJoins) -> Option<ObjectName> {
    match &table.relation {
        TableFactor::Table { name, .. } => Some(name.clone()),
        _ => None,
    }
}
fn validate_alter_operations(
    operations: &[sqlparser::ast::AlterTableOperation],
) -> Result<(), String> {
    for operation in operations {
        let target = match operation {
            sqlparser::ast::AlterTableOperation::RenameTable { table_name }
            | sqlparser::ast::AlterTableOperation::SwapWith { table_name } => Some(table_name),
            _ => None,
        };
        if let Some(target) = target {
            validate_write_target(target)?;
        }
    }
    Ok(())
}
fn write_target_name_from_delete(from: &sqlparser::ast::FromTable) -> Option<ObjectName> {
    let tables = match from {
        sqlparser::ast::FromTable::WithFromKeyword(tables)
        | sqlparser::ast::FromTable::WithoutKeyword(tables) => tables,
    };
    tables.first().and_then(write_target_name_from_table)
}
#[derive(Default)]
struct JoinWarningVisitor {
    has_cross_join: bool,
    has_unconditional_join: bool,
}
impl Visitor for JoinWarningVisitor {
    type Break = ();
    fn pre_visit_query(&mut self, query: &Query) -> ControlFlow<Self::Break> {
        if let SetExpr::Select(select) = query.body.as_ref() {
            for table in &select.from {
                for join in &table.joins {
                    match &join.join_operator {
                        JoinOperator::CrossJoin => self.has_cross_join = true,
                        JoinOperator::Inner(JoinConstraint::None)
                        | JoinOperator::LeftOuter(JoinConstraint::None)
                        | JoinOperator::RightOuter(JoinConstraint::None)
                        | JoinOperator::FullOuter(JoinConstraint::None)
                        | JoinOperator::LeftSemi(JoinConstraint::None)
                        | JoinOperator::RightSemi(JoinConstraint::None)
                        | JoinOperator::LeftAnti(JoinConstraint::None)
                        | JoinOperator::RightAnti(JoinConstraint::None) => {
                            self.has_unconditional_join = true
                        }
                        _ => {}
                    }
                }
            }
        }
        ControlFlow::Continue(())
    }
}
fn join_warning_flags(sql: &str) -> Option<JoinWarningVisitor> {
    let statements = Parser::parse_sql(&sqlparser::dialect::GenericDialect, sql).ok()?;
    let mut visitor = JoinWarningVisitor::default();
    match statements.visit(&mut visitor) {
        ControlFlow::Continue(()) => Some(visitor),
        ControlFlow::Break(()) => None,
    }
}
pub fn generate_warnings(sql: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    let sql_lower = sql.to_lowercase();
    static FILE_NAME_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r#""?([\w\-]+\.(csv|tsv|parquet|pq|xlsx|geojson|json|jsonl|ndjson|shp|gpkg))"?\s*"#,
        )
        .expect("static FILE_NAME_RE is valid")
    });
    if FILE_NAME_RE.is_match(sql) {
        warnings.push(
            "ERROR: Direct file name references like 'data.csv' are not valid table names. \
             Files must be registered via register_data_source and accessed using the \
             vw_ alias view shown in discover_workspace. The alias view name is listed \
             in the query_table field of discover_workspace results."
                .into(),
        );
    }
    if sql_lower.contains("read_csv_auto")
        || sql_lower.contains("read_csv(")
        || sql_lower.contains("read_parquet")
        || sql_lower.contains("read_json_auto")
        || sql_lower.contains("read_xlsx")
        || sql_lower.contains("st_read")
    {
        warnings.push(
            "TIP: Direct file reading functions (read_csv_auto, read_parquet, etc.) \
             bypass the registration system. Consider using register_data_source to \
             register the file and query through its vw_ alias view instead."
                .into(),
        );
    }
    if (sql_lower.contains("read_csv_auto") || sql_lower.contains("read_csv("))
        && !sql_lower.contains("ignore_errors=true")
    {
        warnings.push(
            "WARNING: CSV parsing is strict by default; malformed rows will fail the query. \
             Add ignore_errors=true only if you accept skipped rows, then validate the result."
                .into(),
        );
    }
    if sql_lower.contains("select *") && !sql_lower.contains("count(*)") {
        warnings.push(
            "TIP: consider specifying explicit column names instead of SELECT * \
             for better performance and clarity"
                .into(),
        );
    }
    if let Some(join_flags) = join_warning_flags(sql) {
        if join_flags.has_cross_join {
            warnings.push(
                "WARNING: CROSS JOIN produces a cartesian product and may cause \
                 excessive memory usage or slow queries. Verify this is intentional."
                    .into(),
            );
        } else if join_flags.has_unconditional_join {
            warnings.push(
                "WARNING: JOIN without ON or USING clause may produce a \
                 cartesian product. Add a join condition if this is unintentional."
                    .into(),
            );
        }
    }
    if sql_lower.contains("order by") && !sql_lower.contains("limit") {
        warnings.push(
            "TIP: ORDER BY without LIMIT triggers a full sort. Consider adding \
             LIMIT if you only need the top or bottom N rows."
                .into(),
        );
    }
    warnings
}
#[derive(Debug)]
pub enum SecurityVerdict {
    Allowed {
        sql: String,
        warnings: Vec<String>,
    },
    Denied {
        reason: String,
        hint: Option<String>,
    },
}
fn inspect_sql_text_fallback(
    sql: &str,
    config: &ServerConfig,
    is_read_only: bool,
) -> SecurityVerdict {
    if AstInspector::contains_always_blocked(sql) {
        return SecurityVerdict::Denied {
            reason: "This query contains operations that are always blocked (INSTALL/LOAD).".into(),
            hint: Some("All needed extensions are pre-loaded at startup.".into()),
        };
    }
    for ep in AstInspector::extract_file_paths_fallback(sql) {
        if let Err(e) = path_validator::validate_path(&ep.file_path, &config.workspace_dir) {
            return SecurityVerdict::Denied {
                reason: format!(
                    "Path validation failed for function '{}': {}",
                    ep.function_name, e
                ),
                hint: Some("Verify file path is within the workspace.".into()),
            };
        }
    }
    if AstInspector::contains_attach_detach(sql) {
        return SecurityVerdict::Denied {
            reason: "ATTACH/DETACH is blocked in all modes.".into(),
            hint: Some("Use register_data_source instead.".into()),
        };
    }
    if is_read_only {
        let clean = AstInspector::strip_sql_literals(sql);
        let ddl_keywords =
            regex::Regex::new(r"(?i)\b(?:CREATE|DROP|ALTER|INSERT|UPDATE|DELETE|COPY|TRUNCATE)\b")
                .unwrap();
        if ddl_keywords.is_match(&clean) {
            return SecurityVerdict::Denied {
                reason: "DDL/DML detected in read-only mode.".into(),
                hint: Some("Use execute_scratchpad_sql for write operations.".into()),
            };
        }
    }
    if !is_read_only {
        let clean = AstInspector::strip_sql_literals(sql);
        let ddl_keywords = regex::Regex::new(
            r"(?i)\b(?:CREATE|DROP|ALTER|INSERT|UPDATE|DELETE|COPY|TRUNCATE|MERGE|CALL)\b",
        )
        .unwrap();
        if ddl_keywords.is_match(&clean) {
            return SecurityVerdict::Denied {
                reason: "DDL/DML detected in write mode without parser support.".into(),
                hint: Some(
                    "Use valid SQL syntax that the parser can understand, or use \
                     execute_read_sql for read-only queries."
                        .into(),
                ),
            };
        }
    }
    let warnings = generate_warnings(sql);
    SecurityVerdict::Allowed {
        sql: sql.to_string(),
        warnings,
    }
}
pub fn inspect_sql(sql: &str, config: &ServerConfig, is_read_only: bool) -> SecurityVerdict {
    let dialect = sqlparser::dialect::GenericDialect;
    let parsed_statements = match Parser::parse_sql(&dialect, sql) {
        Ok(statements) => statements,
        Err(_) => {
            return inspect_sql_text_fallback(sql, config, is_read_only);
        }
    };
    let extracted = AstInspector::extract_file_paths_from_statements(parsed_statements.iter());
    for ep in &extracted {
        if let Err(e) = path_validator::validate_path(&ep.file_path, &config.workspace_dir) {
            return SecurityVerdict::Denied {
                reason: format!(
                    "Path validation failed for function '{}': {}",
                    ep.function_name, e
                ),
                hint: Some("Verify file path is within the workspace. Use discover_workspace to see registered sources.".into()),
            };
        }
    }
    if AstInspector::contains_always_blocked(sql) {
        return SecurityVerdict::Denied {
            reason: "This query contains operations that are always blocked (INSTALL/LOAD). \
            Sidecar extensions are pre-loaded by the server; manual extension \
            installation is not supported for security and stability reasons."
                .into(),
            hint: Some(
                "INSTALL/LOAD is blocked. All needed extensions are pre-loaded at startup.".into(),
            ),
        };
    }
    if is_read_only {
        let has_ddl = AstInspector::contains_dangerous_ddl_from_statements(&parsed_statements, sql);
        if has_ddl {
            return SecurityVerdict::Denied {
                reason: "This query contains DDL/DML statements (COPY, DROP, ALTER, INSERT, etc.) \
                that are not allowed in read-only mode. Use execute_scratchpad_sql for \
                write operations in the sandbox.".into(),
                hint: Some("Use execute_scratchpad_sql for writes, or execute_pipeline with kind='write' for multi-step operations.".into()),
            };
        }
    } else {
        if let Err(reason) = AstInspector::validate_write_statements(&parsed_statements) {
            return SecurityVerdict::Denied {
                reason,
                hint: Some(
                    "Use scratchpad INSERT/UPDATE/DELETE, CREATE TABLE, ALTER TABLE, or DROP TABLE on tm_<16 hex> tables only. CREATE/DROP VIEW and SELECT are not allowed."
                        .into(),
                ),
            };
        }
        if let Err(reason) = validate_public_write_relations(sql) {
            return SecurityVerdict::Denied {
                reason,
                hint: Some("Write targets must be unqualified or in ai_scratchpad; read registered data through vw_ aliases.".into()),
            };
        }
        if AstInspector::contains_attach_detach(sql) {
            return SecurityVerdict::Denied {
                reason: "ATTACH/DETACH is blocked in all modes. \
                Attaching external databases is not allowed for security reasons."
                    .into(),
                hint: Some(
                    "Use register_data_source to register external files within the workspace."
                        .into(),
                ),
            };
        }
    }
    static FILE_TABLE_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#""?([\w\-]+\.(csv|tsv|parquet|pq|xlsx|geojson|json|jsonl|ndjson|shp|gpkg))"?\s*(?:(?:WHERE|LIMIT|GROUP|ORDER|HAVING|JOIN|;|$))"#)
            .expect("static FILE_TABLE_RE is valid")
    });
    if FILE_TABLE_RE.is_match(sql) {
        return SecurityVerdict::Denied {
            reason: "Direct file name used as table name is not allowed. \
            Data files must be registered via register_data_source and accessed \
            through their vw_ alias view."
                .into(),
            hint: Some(
                "Use register_data_source to register the file, then call \
            discover_workspace to find the vw_ alias view name (listed in the query_table field). \
            Always use the vw_ alias in SQL queries."
                    .into(),
            ),
        };
    }
    if looks_like_numeric_table(sql) {
        return SecurityVerdict::Denied {
            reason: "Numeric table name used directly is not allowed. \
            The registered view must be accessed through its vw_ alias."
                .into(),
            hint: Some(
                "Use discover_workspace to find the vw_ alias view name. \
            Always use the vw_ alias (listed in query_table) in SQL queries."
                    .into(),
            ),
        };
    }
    let warnings = generate_warnings(sql);
    SecurityVerdict::Allowed {
        sql: sql.to_string(),
        warnings,
    }
}
