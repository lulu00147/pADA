use crate::engine::{execute_streaming, StreamOptions};
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::utils::sql_normalize::strip_sql_comments;
use crate::utils::{engine_err_to_jsonrpc, interpret_duckdb_error};
use regex::Regex;
use std::sync::atomic::Ordering;
use std::sync::LazyLock;
pub fn has_limit_clause(sql: &str) -> bool {
    static LIMIT_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)\bLIMIT\b").expect("static LIMIT regex is valid"));
    static FETCH_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\bFETCH\s+(FIRST|NEXT)\s+\d+\s+ROWS?\s+ONLY\b")
            .expect("static FETCH regex is valid")
    });
    let no_comments = strip_sql_comments(sql);
    let stripped = crate::security::AstInspector::strip_sql_literals(&no_comments);
    LIMIT_RE.is_match(&stripped) || FETCH_RE.is_match(&stripped)
}
pub struct ExecuteReadSqlHandler;
#[async_trait::async_trait]
impl ToolHandler for ExecuteReadSqlHandler {
    fn name(&self) -> &str {
        "execute_read_sql"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "execute_read_sql".to_string(),
            description: "PREREQUISITE: discover_workspace (vw_ view / column names required). Run SELECT on registered vw_<16 hex> views only (read-only). Cannot query table names or tm_* tables — those are write-only; query the published vw_ view instead. COST: 1 DB query. USE: Simple ad-hoc queries on vw_.... CRITICAL: Always specify explicit column names instead of SELECT *.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {"sql": {"type": "string"}, "limit": {"type": "integer", "description": "Max rows to return. Default: stream_limit from config"}, "offset": {"type": "integer"}}, "required": ["sql"]}),
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
        let (safe_sql, warnings) = super::verify_sql(sql, ctx.config, true)?;
        let user_limit = match params.get("limit") {
            Some(value) => Some(value.as_u64().ok_or_else(|| {
                JsonRpcError::invalid_params("limit must be a non-negative integer")
            })?),
            None => None,
        };
        let user_offset = match params.get("offset") {
            Some(value) => value.as_u64().ok_or_else(|| {
                JsonRpcError::invalid_params("offset must be a non-negative integer")
            })?,
            None => 0,
        };
        let configured_limit = ctx.config.stream_limit as u64;
        let effective_limit = user_limit.unwrap_or(configured_limit).min(configured_limit);
        if effective_limit == 0 {
            return Err(JsonRpcError::invalid_params(
                "limit must be greater than 0".to_string(),
            ));
        }
        let normalized_sql = safe_sql.trim().trim_end_matches(';').trim_end();
        let final_sql = if has_limit_clause(normalized_sql) {
            format!(
                "SELECT * FROM ({}) AS pada_limited LIMIT {} OFFSET {}",
                normalized_sql, effective_limit, user_offset
            )
        } else {
            format!(
                "{} LIMIT {} OFFSET {}",
                normalized_sql, effective_limit, user_offset
            )
        };
        super::validate_native_sql(&*ctx.actor, &final_sql, Some(&ctx.query_id)).await?;
        let options = StreamOptions {
            max_rows: effective_limit as usize,
            max_cell_chars: ctx.config.cell_truncate_chars,
            max_columns: ctx.config.max_columns,
            max_result_bytes: ctx.config.max_result_bytes,
        };
        let result = execute_streaming(&*ctx.actor, &final_sql, &options, Some(&ctx.query_id))
            .await
            .map_err(|e| {
                let (msg, suggestion) = interpret_duckdb_error(&e.to_string());
                let enriched = if let Some(s) = suggestion {
                    format!("{} Suggestion: {}", msg, s)
                } else {
                    msg
                };
                JsonRpcError::internal_error(enriched)
            })?;
        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
        let result_val = serde_json::to_value(result).map_err(|e| engine_err_to_jsonrpc(&e))?;
        let response = super::standard_response(
            serde_json::json!({"results": result_val, "query_id": ctx.query_id}),
            warnings,
        );
        Ok(response)
    }
}
