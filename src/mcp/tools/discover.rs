use crate::engine::{execute_streaming, stream_result_to_json_array};
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::security::{inspect_sql, SecurityVerdict};
use crate::utils::{engine_err_to_jsonrpc, load_persisted_views};
use sqlparser::ast::{GroupByExpr, SetExpr, Statement, TableFactor};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::Ordering;
pub fn is_discoverable_table(schema: &str, name: &str) -> bool {
    if schema.eq_ignore_ascii_case("main") {
        return crate::security::is_public_alias(name);
    }
    if schema.eq_ignore_ascii_case("ai_scratchpad") {
        return crate::security::is_public_alias(name);
    }
    true
}
type DatasetMetadata = (String, String, Option<String>, String);
fn view_source_relation(definition: &str) -> Option<(String, String, String)> {
    let mut statements = Parser::parse_sql(&GenericDialect, definition).ok()?;
    if statements.len() != 1 {
        return None;
    }
    let statement = statements.pop()?;
    let Statement::CreateView { query, .. } = statement else {
        return None;
    };
    if query.with.is_some()
        || !query.order_by.is_empty()
        || query.limit.is_some()
        || !query.limit_by.is_empty()
        || query.offset.is_some()
        || query.fetch.is_some()
        || !query.locks.is_empty()
        || query.for_clause.is_some()
    {
        return None;
    }
    let SetExpr::Select(select) = query.body.as_ref() else {
        return None;
    };
    if select.projection.len() != 1
        || select.projection[0].to_string() != "*"
        || select.from.len() != 1
        || !select.from[0].joins.is_empty()
        || select.selection.is_some()
        || select.having.is_some()
        || select.qualify.is_some()
        || !select.sort_by.is_empty()
        || !select.cluster_by.is_empty()
        || !select.distribute_by.is_empty()
        || !select.lateral_views.is_empty()
        || !select.named_window.is_empty()
        || select.top.is_some()
        || !matches!(select.group_by, GroupByExpr::Expressions(ref expressions) if expressions.is_empty())
    {
        return None;
    }
    let [table] = select.from.as_slice() else {
        return None;
    };
    let TableFactor::Table {
        name,
        alias,
        args,
        with_hints,
        version,
        partitions,
    } = &table.relation
    else {
        return None;
    };
    if alias.is_some()
        || args.is_some()
        || !with_hints.is_empty()
        || version.is_some()
        || !partitions.is_empty()
    {
        return None;
    }
    let select_sql = select.to_string();
    match name.0.as_slice() {
        [source] => Some((String::new(), source.value.clone(), select_sql)),
        [schema, source] => Some((schema.value.clone(), source.value.clone(), select_sql)),
        _ => None,
    }
}
fn is_registered_alias_view(
    schema: &str,
    name: &str,
    view_definition: Option<&str>,
    metadata: &HashMap<String, DatasetMetadata>,
) -> bool {
    if !schema.eq_ignore_ascii_case("ai_scratchpad") || !crate::security::is_public_alias(name) {
        return false;
    }
    let Some((_, _, _, source_view)) = metadata.get(name) else {
        return false;
    };
    view_definition.and_then(view_source_relation).is_some_and(
        |(source_schema, source_name, select_sql)| {
            source_name == *source_view
                && select_sql.eq_ignore_ascii_case(&format!("SELECT * FROM {source_view}"))
                && (source_schema.is_empty() || source_schema.eq_ignore_ascii_case("ai_scratchpad"))
        },
    )
}
fn dataset_metadata_by_alias(
    state_dir: &str,
) -> (HashMap<String, DatasetMetadata>, Option<String>) {
    let mut metadata = HashMap::new();
    let entries = match load_persisted_views(Path::new(state_dir)) {
        Ok(entries) => entries,
        Err(_) => {
            return (
                metadata,
                Some("Registered dataset metadata is unavailable".into()),
            )
        }
    };
    for entry in entries {
        let Some(alias) = entry.get("alias_name").and_then(|value| value.as_str()) else {
            continue;
        };
        let Some(file_path) = entry.get("file_path").and_then(|value| value.as_str()) else {
            continue;
        };
        let Some(dataset_name) = Path::new(file_path)
            .file_name()
            .and_then(|value| value.to_str())
        else {
            continue;
        };
        let format = entry
            .get("format")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let sheet_name = entry
            .get("sheet_name")
            .and_then(|value| value.as_str())
            .map(str::to_string);
        let Some(source_view) = entry.get("view_name").and_then(|value| value.as_str()) else {
            continue;
        };
        metadata.insert(
            alias.to_string(),
            (
                dataset_name.to_string(),
                format,
                sheet_name,
                source_view.to_string(),
            ),
        );
    }
    (metadata, None)
}
pub struct DiscoverWorkspaceHandler;
#[async_trait::async_trait]
impl ToolHandler for DiscoverWorkspaceHandler {
    fn name(&self) -> &str {
        "discover_workspace"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "discover_workspace".to_string(),
            description: "One-call workspace exploration: returns every registered table/view with schema and row count. Column stats and sample rows are OFF by default (use summarize_table, or pass include_stats/include_sample true). COST: 1 + N*queries. IMPORTANT: Each registered data source has a query_table field — always use that exact vw_ alias name in SQL queries.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {"include_sample": {"type": "boolean", "description": "Include 1 sample row per table. Default: false"}, "include_stats": {"type": "boolean", "description": "Include SUMMARIZE column statistics (min/max/avg/null%). Default: false; prefer summarize_table for one table."}}}),
        }
    }
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let include_sample = match params.get("include_sample") {
            None => false,
            Some(value) => value
                .as_bool()
                .ok_or_else(|| JsonRpcError::invalid_params("include_sample must be a boolean"))?,
        };
        let include_stats = match params.get("include_stats") {
            None => false,
            Some(value) => value
                .as_bool()
                .ok_or_else(|| JsonRpcError::invalid_params("include_stats must be a boolean"))?,
        };
        let table_sql = r#"
            SELECT t.table_schema, t.table_name, t.table_type, v.view_definition
            FROM information_schema.tables AS t
            LEFT JOIN information_schema.views AS v
              ON v.table_schema = t.table_schema AND v.table_name = t.table_name
            WHERE t.table_schema IN ('main', 'ai_scratchpad')
            ORDER BY table_schema, table_name
        "#;
        let stream_result = execute_streaming(
            ctx.actor,
            table_sql,
            &Default::default(),
            Some(&ctx.query_id),
        )
        .await
        .map_err(|e| engine_err_to_jsonrpc(&e))?;
        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
        let tables: Vec<serde_json::Value> = stream_result_to_json_array(stream_result);
        let (dataset_metadata, metadata_warning) = dataset_metadata_by_alias(&ctx.config.state_dir);
        let registered_aliases: std::collections::HashSet<&str> = tables
            .iter()
            .filter_map(|table| {
                let schema = table.get("table_schema")?.as_str()?;
                let name = table.get("table_name")?.as_str()?;
                let definition = table
                    .get("view_definition")
                    .and_then(|value| value.as_str());
                is_registered_alias_view(schema, name, definition, &dataset_metadata)
                    .then_some(name)
            })
            .collect();
        let table_entries: Vec<(&str, &str, &str, Option<&str>)> = tables
            .iter()
            .filter_map(|r| {
                let schema = r.get("table_schema")?.as_str()?;
                let name = r.get("table_name")?.as_str()?;
                let table_type = r.get("table_type")?.as_str()?;
                let view_definition = r.get("view_definition").and_then(|value| value.as_str());
                if !is_discoverable_table(schema, name)
                    || (schema.eq_ignore_ascii_case("main") && registered_aliases.contains(name))
                {
                    return None;
                }
                Some((schema, name, table_type, view_definition))
            })
            .collect();
        if table_entries.is_empty() {
            let resp = super::standard_response(
                serde_json::json!({"tables": [], "total_table_count": 0, "hint": "No tables registered yet. Use register_data_source with a known CSV/Parquet/GeoJSON path, or upload a file through the HTTP transport.", "usage_guide": "After registering a data source, call discover_workspace again. Query only vw_<16 hex> views via execute_read_sql / pipeline kind=query / summarize_table. tm_<16 hex> tables are write-only (CREATE/INSERT/UPDATE/DELETE/DROP TABLE via execute_scratchpad_sql); CREATE TABLE auto-registers vw_<same hex>. Never query table names. Never use schema-qualified ai_scratchpad.vw_..."}),
                metadata_warning.into_iter().collect(),
            );
            return Ok(resp);
        }
        let mut result_tables: Vec<serde_json::Value> = Vec::with_capacity(table_entries.len());
        let mut warnings: Vec<String> = metadata_warning.into_iter().collect();
        let mut total_rows = 0usize;
        let mut total_cols = 0usize;
        for (schema, name, table_type, view_definition) in &table_entries {
            let qualified = if *schema == "ai_scratchpad" {
                format!("ai_scratchpad.{name}", name = name)
            } else {
                format!("main.{name}", name = name)
            };
            let query_table = format!("\"{}\".\"{}\"", schema, name);
            let (columns, column_stats): (Vec<serde_json::Value>, Option<Vec<serde_json::Value>>) =
                if include_stats {
                    let summarize_sql = format!(
                        "SUMMARIZE SELECT * FROM \"{}\".\"{}\"",
                        schema.replace('"', "\"\""),
                        name.replace('"', "\"\"")
                    );
                    let verdict = inspect_sql(&summarize_sql, ctx.config, true);
                    let safe_sql = match &verdict {
                        SecurityVerdict::Allowed { sql, warnings: w } => {
                            if !w.is_empty() {
                                warnings.extend(w.iter().cloned());
                            }
                            sql.clone()
                        }
                        SecurityVerdict::Denied { reason, .. } => {
                            warnings.push(format!("SUMMARIZE {qualified} denied: {reason}"));
                            String::new()
                        }
                    };
                    if safe_sql.is_empty() {
                        (Vec::new(), None)
                    } else {
                        match execute_streaming(
                            ctx.actor,
                            &safe_sql,
                            &Default::default(),
                            Some(&ctx.query_id),
                        )
                        .await
                        {
                            Ok(stream) => {
                                ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
                                let parsed = stream_result_to_json_array(stream);
                                if parsed.is_empty() {
                                    (Vec::new(), None)
                                } else {
                                    let allowed_stats = [
                                        "column_name",
                                        "column_type",
                                        "min",
                                        "max",
                                        "approx_unique",
                                        "avg",
                                        "std",
                                        "q25",
                                        "q50",
                                        "q75",
                                        "count",
                                        "null_percentage",
                                    ];
                                    let cols: Vec<serde_json::Value> = parsed
                                        .iter()
                                        .map(|row| {
                                            let mut slim = serde_json::json!({});
                                            if let Some(n) =
                                                row.get("column_name").and_then(|v| v.as_str())
                                            {
                                                slim["name"] = serde_json::json!(n);
                                            }
                                            if let Some(t) =
                                                row.get("column_type").and_then(|v| v.as_str())
                                            {
                                                slim["type"] = serde_json::json!(t);
                                            }
                                            slim
                                        })
                                        .collect();
                                    let stats: Vec<serde_json::Value> = parsed
                                        .iter()
                                        .map(|row| {
                                            let mut slim = serde_json::Map::new();
                                            for key in allowed_stats {
                                                if let Some(value) = row.get(key) {
                                                    slim.insert(key.to_string(), value.clone());
                                                }
                                            }
                                            serde_json::Value::Object(slim)
                                        })
                                        .collect();
                                    (cols, Some(stats))
                                }
                            }
                            Err(e) => {
                                tracing::debug!(target: "pada::discover", table = %qualified, error = %e, "SUMMARIZE failed");
                                warnings.push(format!(
                                "Statistics unavailable for {name}; column discovery continued."
                            ));
                                (Vec::new(), None)
                            }
                        }
                    }
                } else {
                    (Vec::new(), None)
                };
            let columns = if columns.is_empty() {
                let describe_sql = format!(
                    "DESCRIBE \"{}\".\"{}\"",
                    schema.replace('"', "\"\""),
                    name.replace('"', "\"\"")
                );
                let cols_json = match execute_streaming(
                    ctx.actor,
                    &describe_sql,
                    &Default::default(),
                    Some(&ctx.query_id),
                )
                .await
                {
                    Ok(stream) => {
                        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
                        stream_result_to_json_array(stream)
                    }
                    Err(e) => {
                        tracing::debug!(target: "pada::discover", table = %qualified, error = %e, "DESCRIBE failed");
                        warnings.push(format!(
                            "Column discovery unavailable for {name}; this dataset was skipped."
                        ));
                        continue;
                    }
                };
                cols_json
                    .into_iter()
                    .map(|col| {
                        let mut slim = serde_json::json!({});
                        if let Some(n) = col.get("column_name").and_then(|v| v.as_str()) {
                            slim["name"] = serde_json::json!(n);
                        }
                        if let Some(t) = col
                            .get("column_type")
                            .or(col.get("columntype"))
                            .and_then(|v| v.as_str())
                        {
                            slim["type"] = serde_json::json!(t);
                        }
                        slim
                    })
                    .collect()
            } else {
                columns
            };
            let count_sql = format!(
                "SELECT COUNT(*) AS cnt FROM \"{}\".\"{}\"",
                schema.replace('"', "\"\""),
                name.replace('"', "\"\"")
            );
            let (row_count, healthy) = match execute_streaming(
                ctx.actor,
                &count_sql,
                &Default::default(),
                Some(&ctx.query_id),
            )
            .await
            {
                Ok(stream) => {
                    ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
                    let rows = stream_result_to_json_array(stream);
                    let cnt = rows
                        .first()
                        .and_then(|r| r.get("cnt"))
                        .and_then(|v| {
                            v.as_u64()
                                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                        })
                        .unwrap_or(0) as usize;
                    (cnt, true)
                }
                Err(e) => {
                    tracing::debug!(target: "pada::discover", table = %qualified, error = %e, "COUNT failed");
                    warnings.push(format!(
                        "Data source '{}' is inaccessible; it may point to a deleted or moved file.",
                        qualified
                    ));
                    (0, false)
                }
            };
            total_rows += row_count;
            let col_count = columns.len();
            total_cols += col_count;
            let sample = if include_sample && row_count > 0 {
                match execute_streaming(
                    ctx.actor,
                    &format!("SELECT * FROM \"{}\".\"{}\" LIMIT 1", schema, name),
                    &Default::default(),
                    Some(&ctx.query_id),
                )
                .await
                {
                    Ok(stream) => {
                        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
                        stream_result_to_json_array(stream).into_iter().next()
                    }
                    Err(_) => None,
                }
            } else {
                None
            };
            let mut table = serde_json::json!({
                "name": name,
                "query_table": query_table,
                "row_count": row_count,
                "healthy": healthy,
                "column_count": col_count,
                "columns": columns,
                "column_stats": column_stats,
                "sample_row": sample,
            });
            if crate::security::is_public_alias(name) && *table_type == "VIEW" {
                if let Some((dataset_name, format, sheet_name, source_view)) =
                    dataset_metadata.get(*name)
                {
                    let expected_select = format!("SELECT * FROM {source_view}");
                    let matches_registry = view_definition
                        .and_then(view_source_relation)
                        .is_some_and(|(source_schema, source_name, select_sql)| {
                            source_name == *source_view
                                && select_sql.eq_ignore_ascii_case(&expected_select)
                                && (source_schema.is_empty()
                                    || source_schema.eq_ignore_ascii_case(schema))
                        });
                    if matches_registry {
                        table["dataset_name"] = serde_json::json!(dataset_name);
                        table["format"] = serde_json::json!(format);
                        if let Some(sheet_name) = sheet_name {
                            table["sheet_name"] = serde_json::json!(sheet_name);
                        }
                    }
                }
            }
            result_tables.push(table);
        }
        let resp = super::standard_response(
            serde_json::json!({
                "tables": result_tables,
                "total_table_count": result_tables.len(),
                "total_rows": total_rows,
                "total_columns": total_cols,
                "usage_guide": "Query only vw_<16 hex> views via execute_read_sql / pipeline kind=query / summarize_table. tm_<16 hex> tables are write-only (CREATE/INSERT/UPDATE/DELETE/DROP TABLE via execute_scratchpad_sql); CREATE TABLE auto-registers vw_<same hex>. Never query table names. Never use schema-qualified ai_scratchpad.vw_...",
            }),
            warnings,
        );
        Ok(resp)
    }
}
