use crate::config::ServerConfig;
use crate::engine::TimedActor;
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::ToolHandler;
use crate::security::SecurityVerdict;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::sync::atomic::AtomicU64;
use std::time::Instant;
pub const MAX_UPLOAD_BYTES: u64 = 500 * 1024 * 1024;
pub mod register;
pub use register::*;
pub mod read;
pub use read::*;
pub mod scratchpad;
pub use scratchpad::*;
pub mod misc;
pub use misc::*;
pub mod summarize;
pub use summarize::*;
pub mod pipeline;
pub use pipeline::*;
pub mod explain;
pub use explain::*;
pub mod discover;
pub use crate::mcp::protocol::ToolDefinition;
pub use crate::utils::sql_normalize::escape_ident;
pub use discover::*;
pub struct Metrics {
    pub start_time: Instant,
    pub queries: AtomicU64,
}
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for Metrics {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            queries: AtomicU64::new(0),
        }
    }
}
pub fn standard_response(data: serde_json::Value, warnings: Vec<String>) -> serde_json::Value {
    let mut resp = serde_json::json!({
        "success": true,
        "data": data,
    });
    if !warnings.is_empty() {
        resp.as_object_mut()
            .expect("just-created json is an object")
            .insert("warnings".into(), serde_json::json!(warnings));
    }
    resp
}
pub fn unpack_verdict(verdict: &SecurityVerdict) -> Result<(String, Vec<String>), JsonRpcError> {
    match verdict {
        SecurityVerdict::Allowed { sql, warnings } => Ok((sql.clone(), warnings.clone())),
        SecurityVerdict::Denied { reason, hint } => {
            let msg = if let Some(h) = hint {
                format!("{}. Hint: {}", reason, h)
            } else {
                reason.clone()
            };
            Err(JsonRpcError::invalid_params(msg))
        }
    }
}
pub fn verify_sql(
    sql: &str,
    config: &ServerConfig,
    is_read_only: bool,
) -> Result<(String, Vec<String>), JsonRpcError> {
    let verdict = crate::security::inspect_sql(sql, config, is_read_only);
    let result = unpack_verdict(&verdict)?;
    if Parser::parse_sql(&GenericDialect, sql).is_ok() {
        let relation_result = if is_read_only {
            crate::security::validate_public_read_relations(sql)
        } else {
            crate::security::validate_public_write_relations(sql)
        };
        relation_result.map_err(JsonRpcError::invalid_params)?;
    }
    Ok(result)
}
fn native_prepare_error(engine_error: impl std::fmt::Display) -> JsonRpcError {
    let full = engine_error.to_string();
    let detail = full
        .split_once("cannot prepare this SQL")
        .map(|(_, rest)| rest.trim_start_matches([':', ' ']))
        .filter(|d| !d.is_empty())
        .unwrap_or(full.as_str());
    JsonRpcError::invalid_params(format!("DuckDB cannot prepare this SQL: {detail}"))
}
pub async fn validate_native_sql(
    actor: &TimedActor,
    sql: &str,
    query_id: Option<&str>,
) -> Result<(), JsonRpcError> {
    actor
        .validate_native(sql, query_id)
        .await
        .map_err(native_prepare_error)
}
pub fn is_single_read_only_query(sql: &str) -> bool {
    matches!(Parser::parse_sql(&GenericDialect, sql), Ok(statements) if statements.len() == 1)
        && is_read_only_query(sql)
}
pub fn default_handlers() -> Vec<Box<dyn ToolHandler>> {
    vec![
        Box::new(RegisterDataSourceHandler),
        Box::new(ExecuteReadSqlHandler),
        Box::new(ExecuteScratchpadSqlHandler),
        Box::new(GetServerMetricsHandler),
        Box::new(SummarizeTableHandler),
        Box::new(ExecutePipelineHandler),
        Box::new(ExplainQueryHandler),
        Box::new(DiscoverWorkspaceHandler),
    ]
}
