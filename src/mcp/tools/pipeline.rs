use crate::engine::StreamOptions;
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::utils::engine_err_to_jsonrpc;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::sync::atomic::Ordering;
const MAX_PIPELINE_STEPS: usize = 100;
const MAX_PIPELINE_SQL_CHARS: usize = 1024 * 1024;
const MAX_PIPELINE_SQL_TOTAL_BYTES: usize = 4 * 1024 * 1024;
const MAX_PIPELINE_TOTAL_ROWS: usize = 50_000;
const MAX_PIPELINE_RESULT_BYTES: usize = 32 * 1024 * 1024;
pub struct ExecutePipelineHandler;
#[async_trait::async_trait]
impl ToolHandler for ExecutePipelineHandler {
    fn name(&self) -> &str {
        "execute_pipeline"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "execute_pipeline".to_string(),
            description: "Execute multiple SQL steps in ONE call. kind=write: CREATE/INSERT/UPDATE/DELETE/ALTER/DROP TABLE on tm_<16 hex> tables only; CREATE TABLE auto-registers VIEW vw_<same hex>. kind=query: SELECT on vw_... views ONLY — cannot read tm_* tables; query the published vw_ view instead. COST: 1 DB query per step. USE: Multi-step write workflows. DO NOT USE: For a single SELECT (use execute_read_sql).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "steps": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "sql": {"type": "string"},
                                "name": {"type": "string"},
                                "kind": {
                                    "type": "string",
                                    "enum": ["auto", "query", "write"],
                                    "description": "auto: detect from SQL; query: use read actor; write: use write actor"
                                }
                            },
                            "required": ["sql"]
                        }
                    }
                },
                "required": ["steps"]
            }),
        }
    }
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let steps = params
            .get("steps")
            .and_then(|v| v.as_array())
            .filter(|steps| !steps.is_empty())
            .ok_or_else(|| {
                JsonRpcError::invalid_params(
                    "Missing or invalid 'steps': must be a non-empty array of step objects.",
                )
            })?;
        if steps.len() > MAX_PIPELINE_STEPS {
            return Err(JsonRpcError::invalid_params(format!(
                "Too many pipeline steps: maximum is {}",
                MAX_PIPELINE_STEPS
            )));
        }
        let mut results: Vec<serde_json::Value> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut total_sql_bytes = 0usize;
        let mut total_rows = 0usize;
        let mut total_result_bytes = 0usize;
        for (i, step) in steps.iter().enumerate() {
            let sql = step.get("sql").and_then(|v| v.as_str()).ok_or_else(|| {
                JsonRpcError::invalid_params(format!("Step {}: missing 'sql' field", i))
            })?;
            if sql.len() > MAX_PIPELINE_SQL_CHARS {
                return Err(JsonRpcError::invalid_params(format!(
                    "Step {} SQL is too large: maximum is {} bytes",
                    i, MAX_PIPELINE_SQL_CHARS
                )));
            }
            total_sql_bytes = total_sql_bytes.saturating_add(sql.len());
            if total_sql_bytes > MAX_PIPELINE_SQL_TOTAL_BYTES {
                return Err(JsonRpcError::invalid_params(format!(
                    "Pipeline SQL is too large: maximum total is {} bytes",
                    MAX_PIPELINE_SQL_TOTAL_BYTES
                )));
            }
            let kind = step.get("kind").and_then(|v| v.as_str()).unwrap_or("auto");
            let is_write = match kind {
                "write" => true,
                "query" => false,
                "auto" => !is_read_only_query(sql),
                other => {
                    return Err(JsonRpcError::invalid_params(format!(
                        "Step {}: invalid kind '{}'; expected auto, query, or write",
                        i, other
                    )))
                }
            };
            if is_write {
                let (safe_sql, step_warnings) = super::verify_sql(sql, ctx.config, false)?;
                warnings.extend(step_warnings);
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
                let alias_sync = super::scratchpad::sync_scratchpad_aliases(&safe_sql, ctx).await;
                if !alias_sync.failures.is_empty() {
                    return Err(super::scratchpad::aliases_not_published_error(
                        &alias_sync.published,
                        &alias_sync.failures,
                    ));
                }
                warnings.extend(alias_sync.warnings);
                let mut step_result =
                    serde_json::json!({"step": i, "kind": "write", "result": result});
                if !alias_sync.published.is_empty() {
                    step_result["published_aliases"] = serde_json::json!(alias_sync.published);
                }
                total_result_bytes = total_result_bytes.saturating_add(
                    serde_json::to_vec(&step_result)
                        .map_err(|_| {
                            JsonRpcError::internal_error("Pipeline result serialization failed")
                        })?
                        .len(),
                );
                if total_result_bytes > MAX_PIPELINE_RESULT_BYTES {
                    return Err(JsonRpcError::invalid_params(format!(
                        "Pipeline results are too large: maximum total is {} bytes",
                        MAX_PIPELINE_RESULT_BYTES
                    )));
                }
                results.push(step_result);
            } else {
                let (safe_sql, step_warnings) = super::verify_sql(sql, ctx.config, true)?;
                warnings.extend(step_warnings);
                let options = StreamOptions {
                    max_rows: ctx.config.stream_limit,
                    max_cell_chars: ctx.config.cell_truncate_chars,
                    max_columns: ctx.config.max_columns,
                    max_result_bytes: ctx.config.max_result_bytes,
                };
                let stream = ctx
                    .actor
                    .read_structured_stream_on_write(&safe_sql, Some(&ctx.query_id))
                    .await
                    .map_err(|e| engine_err_to_jsonrpc(&e))?;
                let stream_result = crate::engine::consume_streaming(&*ctx.actor, stream, &options)
                    .await
                    .map_err(|e| engine_err_to_jsonrpc(&e))?;
                ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
                let step_row_count = stream_result.row_count;
                let step_truncated = stream_result.truncated;
                let val = crate::engine::stream_result_to_json_array(stream_result);
                total_rows = total_rows.saturating_add(val.len());
                if total_rows > MAX_PIPELINE_TOTAL_ROWS {
                    return Err(JsonRpcError::invalid_params(format!(
                        "Pipeline results contain too many rows: maximum total is {}",
                        MAX_PIPELINE_TOTAL_ROWS
                    )));
                }
                let step_result = serde_json::json!({
                    "step": i,
                    "kind": "query",
                    "data": val,
                    "row_count": step_row_count,
                    "truncated": step_truncated,
                });
                total_result_bytes = total_result_bytes.saturating_add(
                    serde_json::to_vec(&step_result)
                        .map_err(|_| {
                            JsonRpcError::internal_error("Pipeline result serialization failed")
                        })?
                        .len(),
                );
                if total_result_bytes > MAX_PIPELINE_RESULT_BYTES {
                    return Err(JsonRpcError::invalid_params(format!(
                        "Pipeline results are too large: maximum total is {} bytes",
                        MAX_PIPELINE_RESULT_BYTES
                    )));
                }
                results.push(step_result);
            }
        }
        Ok(super::standard_response(
            serde_json::json!({"steps": results}),
            warnings,
        ))
    }
}
fn is_read_only_query_ast(query: &sqlparser::ast::Query) -> bool {
    query.with.as_ref().is_none_or(|with| {
        with.cte_tables
            .iter()
            .all(|cte| is_read_only_query_ast(&cte.query))
    }) && is_read_only_set_expr(&query.body)
}
fn is_read_only_set_expr(expr: &sqlparser::ast::SetExpr) -> bool {
    match expr {
        sqlparser::ast::SetExpr::Select(_) | sqlparser::ast::SetExpr::Values(_) => true,
        sqlparser::ast::SetExpr::Query(query) => is_read_only_query_ast(query),
        sqlparser::ast::SetExpr::SetOperation { left, right, .. } => {
            is_read_only_set_expr(left) && is_read_only_set_expr(right)
        }
        sqlparser::ast::SetExpr::Table(_) => true,
        sqlparser::ast::SetExpr::Insert(_) | sqlparser::ast::SetExpr::Update(_) => false,
    }
}
pub fn is_read_only_query(sql: &str) -> bool {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return true;
    }
    let dialect = GenericDialect;
    match Parser::parse_sql(&dialect, trimmed) {
        Ok(statements) if !statements.is_empty() => {
            for stmt in &statements {
                let ok = match stmt {
                    sqlparser::ast::Statement::Query(query) => is_read_only_query_ast(query),
                    sqlparser::ast::Statement::Explain { .. }
                    | sqlparser::ast::Statement::ExplainTable { .. }
                    | sqlparser::ast::Statement::ShowVariable { .. }
                    | sqlparser::ast::Statement::ShowVariables { .. }
                    | sqlparser::ast::Statement::ShowColumns { .. }
                    | sqlparser::ast::Statement::ShowTables { .. }
                    | sqlparser::ast::Statement::ShowCreate { .. }
                    | sqlparser::ast::Statement::ShowFunctions { .. }
                    | sqlparser::ast::Statement::ShowStatus { .. } => true,
                    _ => false,
                };
                if !ok {
                    return false;
                }
            }
            true
        }
        _ => {
            let upper = trimmed.to_uppercase().trim().to_string();
            let first_word = upper.split_whitespace().next().unwrap_or("");
            let known_read_only = [
                "SUMMARIZE",
                "SELECT",
                "EXPLAIN",
                "SHOW",
                "DESCRIBE",
                "VALUES",
                "PRAGMA",
            ];
            known_read_only.contains(&first_word)
        }
    }
}
