use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::security::public_alias::is_scratchpad_table;
use crate::utils::{engine_err_to_jsonrpc, escape_ident, load_persisted_views};
use sqlparser::ast::{ObjectName, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::path::Path;
use std::sync::atomic::Ordering;
pub struct ExecuteScratchpadSqlHandler;
#[async_trait::async_trait]
impl ToolHandler for ExecuteScratchpadSqlHandler {
    fn name(&self) -> &str {
        "execute_scratchpad_sql"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "execute_scratchpad_sql".to_string(),
            description: "Write-only SQL on ai_scratchpad TABLES named tm_<16 hex> (19 chars, like tm_0123456789abcdef). USE: CREATE/INSERT/UPDATE/DELETE/ALTER/DROP TABLE only. Successful CREATE TABLE auto-registers a public VIEW alias vw_<derived 16 hex> (NOT the table's hex: it is derived from the table name the same way file registrations derive theirs) for execute_read_sql / pipeline kind=query; read the alias from the returned `published_aliases`. FROM registered vw_... views. Do NOT CREATE/DROP VIEW. Do NOT SELECT (cannot read tm_* tables). Do NOT write other names, ai_scratchpad.tm_..., CREATE OR REPLACE, or CREATE TEMP. COST: 1 DB query.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {"sql": {"type": "string"}}, "required": ["sql"]}),
        }
    }
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let sql = params
            .get("sql")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError::invalid_params("Missing required parameter: sql"))?;
        const MAX_SQL_BYTES: usize = 1_000_000;
        if sql.len() > MAX_SQL_BYTES {
            return Err(JsonRpcError::invalid_params(format!(
                "SQL too large: {} bytes (max {})",
                sql.len(),
                MAX_SQL_BYTES
            )));
        }
        let (safe_sql, mut warnings) = super::verify_sql(sql, ctx.config, false)?;
        ctx.actor
            .validate_native_on_write(&safe_sql, Some(&ctx.query_id))
            .await
            .map_err(super::native_prepare_error)?;
        let result = ctx
            .actor
            .write(&safe_sql, Some(&ctx.query_id))
            .await
            .map_err(|e| engine_err_to_jsonrpc(&e))?;
        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
        let alias_sync = sync_scratchpad_aliases(&safe_sql, ctx).await;
        if !alias_sync.failures.is_empty() {
            return Err(aliases_not_published_error(
                &alias_sync.published,
                &alias_sync.failures,
            ));
        }
        warnings.extend(alias_sync.warnings);
        let result_value: serde_json::Value = serde_json::from_str(&result).unwrap_or_else(|e| {
            tracing::warn!("scratchpad: JSON parse error, using raw string: {}", e);
            serde_json::Value::String(result)
        });
        let mut data = serde_json::json!({"results": result_value});
        if !alias_sync.published.is_empty() {
            data["published_aliases"] = serde_json::json!(alias_sync.published);
        }
        Ok(super::standard_response(data, warnings))
    }
}
fn vw_alias_for_tm(tm: &str) -> Option<String> {
    is_scratchpad_table(tm).then(|| crate::engine::generate_short_alias(tm))
}
fn alias_view_sql_for(alias: &str, tm: &str) -> String {
    format!(
        "CREATE OR REPLACE VIEW {} AS SELECT * FROM {};",
        escape_ident(alias),
        escape_ident(tm),
    )
}
fn scratchpad_object_name(name: &ObjectName) -> Option<String> {
    let parts: Vec<String> = name.0.iter().map(|part| part.value.clone()).collect();
    match parts.as_slice() {
        [object] if is_scratchpad_table(object) => Some(object.clone()),
        [schema, object]
            if schema.eq_ignore_ascii_case("ai_scratchpad") && is_scratchpad_table(object) =>
        {
            Some(object.clone())
        }
        _ => None,
    }
}
fn parse_scratchpad_sql(sql: &str) -> Vec<Statement> {
    Parser::parse_sql(&GenericDialect, sql).unwrap_or_default()
}
fn scratchpad_creates(sql: &str) -> Vec<String> {
    net_scratchpad_ops(sql).0
}
fn scratchpad_drops(sql: &str) -> Vec<String> {
    net_scratchpad_ops(sql).1
}
fn net_scratchpad_ops(sql: &str) -> (Vec<String>, Vec<String>) {
    let mut live: Vec<String> = Vec::new();
    let mut dropped: Vec<String> = Vec::new();
    for statement in parse_scratchpad_sql(sql) {
        match statement {
            Statement::CreateTable { name, .. } => {
                if let Some(tm) = scratchpad_object_name(&name) {
                    dropped.retain(|existing| existing != &tm);
                    if !live.iter().any(|existing| existing == &tm) {
                        live.push(tm);
                    }
                }
            }
            Statement::Drop {
                object_type: sqlparser::ast::ObjectType::Table,
                names,
                ..
            } => {
                for name in names {
                    if let Some(tm) = scratchpad_object_name(&name) {
                        live.retain(|existing| existing != &tm);
                        if !dropped.iter().any(|existing| existing == &tm) {
                            dropped.push(tm);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (live, dropped)
}
fn persisted_alias_taken(state_dir: &str, alias: &str) -> Result<bool, String> {
    let entries = load_persisted_views(Path::new(state_dir))?;
    Ok(entries
        .iter()
        .any(|entry| entry.get("alias_name").and_then(|value| value.as_str()) == Some(alias)))
}
pub(crate) fn aliases_not_published_error(
    published: &[String],
    failures: &[String],
) -> JsonRpcError {
    JsonRpcError {
        code: -32602,
        message: "Invalid params".into(),
        data: Some(serde_json::json!({
            "detail": format!(
                "SQL executed, but the public read alias was NOT published ({} of {} failed): {}. Settlement: the read path only accepts vw_<16 hex>; until this is fixed, do not query this table. Fix: if the alias is already taken by a file data source, CREATE a table with a different tm_<16 hex>; otherwise retry DROP + CREATE.",
                failures.len(),
                published.len() + failures.len(),
                failures.join("; ")
            ),
            "published_aliases": published.to_vec(),
            "failed_aliases": failures.to_vec(),
        })),
    }
}
pub(crate) struct AliasSyncOutcome {
    pub(crate) published: Vec<String>,
    pub(crate) warnings: Vec<String>,
    pub(crate) failures: Vec<String>,
}
pub(crate) async fn sync_scratchpad_aliases(
    sql: &str,
    ctx: &HandlerContext<'_>,
) -> AliasSyncOutcome {
    let mut aliases = Vec::new();
    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    for tm in scratchpad_creates(sql) {
        ctx.actor.note_temp_table_created(&tm);
        let Some(alias) = vw_alias_for_tm(&tm) else {
            continue;
        };
        match persisted_alias_taken(&ctx.config.state_dir, &alias) {
            Ok(true) => {
                let msg = format!(
                    "Did not publish {alias}: already registered as a file data source. Choose a different tm_<16 hex>."
                );
                tracing::warn!("scratchpad: {msg}");
                warnings.push(msg);
                continue;
            }
            Err(error) => {
                let msg = format!(
                    "Failed to publish {alias} for {tm}: cannot read view registry ({error})"
                );
                tracing::error!("scratchpad: {msg}");
                failures.push(msg);
                continue;
            }
            Ok(false) => {}
        }
        let view_sql = alias_view_sql_for(&alias, &tm);
        if let Err(error) = ctx
            .actor
            .register_view(&alias, &view_sql, Some(&ctx.query_id))
            .await
        {
            let msg = format!("Failed to publish {alias} for {tm}: {error}");
            tracing::error!("scratchpad: {msg}");
            failures.push(msg);
            continue;
        }
        aliases.push(alias);
    }
    for tm in scratchpad_drops(sql) {
        ctx.actor.note_temp_table_dropped(&tm);
        let Some(alias) = vw_alias_for_tm(&tm) else {
            continue;
        };
        if persisted_alias_taken(&ctx.config.state_dir, &alias).unwrap_or(true) {
            continue;
        }
        let drop_sql = format!("DROP VIEW IF EXISTS {}", escape_ident(&alias));
        if let Err(error) = ctx
            .actor
            .register_view(&alias, &drop_sql, Some(&ctx.query_id))
            .await
        {
            warnings.push(format!("Failed to drop published alias {alias}: {error}"));
        }
    }
    AliasSyncOutcome {
        published: aliases,
        warnings,
        failures,
    }
}
pub async fn expire_scratchpad_tables(
    actor: &crate::engine::TimedActor,
    state_dir: &str,
    ttl: std::time::Duration,
) -> (usize, Vec<String>) {
    let mut dropped = 0usize;
    let mut warnings = Vec::new();
    for (tm, created) in actor.expired_temp_tables(ttl) {
        if !actor.temp_table_is_current(&tm, created) {
            continue;
        }
        let drop_table = format!("DROP TABLE IF EXISTS {};", escape_ident(&tm));
        if let Err(error) = actor.write(&drop_table, None).await {
            tracing::warn!("TTL: cannot drop expired scratchpad table {tm}: {error}");
            warnings.push(format!("Failed to expire scratchpad table {tm}: {error}"));
            continue;
        }
        actor.note_temp_table_dropped(&tm);
        dropped += 1;
        tracing::info!("TTL expired scratchpad table {tm}");
        let Some(alias) = vw_alias_for_tm(&tm) else {
            continue;
        };
        if persisted_alias_taken(state_dir, &alias).unwrap_or(true) {
            continue;
        }
        let drop_view = format!("DROP VIEW IF EXISTS {}", escape_ident(&alias));
        if let Err(error) = actor.register_view(&alias, &drop_view, None).await {
            warnings.push(format!("Failed to drop published alias {alias}: {error}"));
        }
    }
    (dropped, warnings)
}
