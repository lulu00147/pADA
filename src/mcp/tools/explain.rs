use crate::engine::{execute_streaming, stream_result_to_json_array};
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::utils::engine_err_to_jsonrpc;
use std::sync::atomic::Ordering;
pub struct ExplainQueryHandler;
#[async_trait::async_trait]
impl ToolHandler for ExplainQueryHandler {
    fn name(&self) -> &str {
        "explain_query"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "explain_query".to_string(),
            description: "Get query plan WITHOUT executing SQL – safe to use even for DDL/DML statements. COST: 1 cheap DB query. USE: Before expensive queries – catch full table scans, missing indexes, or syntax errors early. The response includes optimization tips.".to_string(),
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
        let (safe_sql, warnings) =
            super::verify_sql(sql, ctx.config, super::is_read_only_query(sql))?;
        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
        let explain_sql = format!("EXPLAIN {}", safe_sql);
        super::validate_native_sql(&*ctx.actor, &explain_sql, Some(&ctx.query_id)).await?;
        let plan_json = serde_json::Value::Array(stream_result_to_json_array(
            execute_streaming(
                &*ctx.actor,
                &explain_sql,
                &Default::default(),
                Some(&ctx.query_id),
            )
            .await
            .map_err(|e| engine_err_to_jsonrpc(&e))?,
        ));
        let plan_text = extract_plan_text(&plan_json);
        let tips = analyze_plan_for_tips(plan_text);
        let sanitized_plan = sanitize_plan_text(plan_text);
        Ok(super::standard_response(
            serde_json::json!({
                "plan_text": sanitized_plan,
                "tips": tips,
            }),
            warnings,
        ))
    }
}
fn extract_plan_text(plan_json: &serde_json::Value) -> &str {
    plan_json
        .as_array()
        .and_then(|rows| {
            rows.iter().find_map(
                |row| match row.get("explain_key").and_then(|v| v.as_str()) {
                    Some("plan") | Some("physical_plan") => {
                        row.get("explain_value").and_then(|v| v.as_str())
                    }
                    _ => row.get("physical_plan").and_then(|v| v.as_str()),
                },
            )
        })
        .unwrap_or("")
}
fn sanitize_plan_text(plan: &str) -> String {
    let mut sanitized = String::with_capacity(plan.len());
    let mut in_string = false;
    for ch in plan.chars() {
        if ch == '\'' {
            in_string = !in_string;
            sanitized.push(ch);
        } else if in_string {
            sanitized.push_str("<redacted>");
        } else {
            sanitized.push(ch);
        }
    }
    sanitized
}
fn analyze_plan_for_tips(plan: &str) -> Vec<String> {
    let mut tips: Vec<String> = Vec::new();
    let lower = plan.to_lowercase();
    if lower.contains("seq_scan") {
        let scan_count = lower.matches("seq_scan").count();
        if scan_count > 1 {
            tips.push(format!(
                "{} tables are being fully scanned (Seq Scan). Consider adding WHERE clauses to filter early.",
                scan_count
            ));
        }
    }
    if lower.contains("cross_product") || lower.contains("cross_join") {
        tips.push("Cross join detected! This may produce huge result sets. Consider adding JOIN conditions.".to_string());
    }
    if lower.contains("hash_join") {
        if lower.contains("perfect_hash_join") {
            tips.push("Perfect Hash Join used -- good performance expected.".to_string());
        } else {
            tips.push("Hash Join detected. For large tables, ensure the smaller table is on the build side.".to_string());
        }
    }
    if lower.contains("order_by") && !lower.contains("top_n") && !lower.contains("limit") {
        tips.push(
            "ORDER BY without LIMIT sorts the entire result set. Add LIMIT for better performance."
                .to_string(),
        );
    }
    if lower.contains("window") && lower.contains("window_scanner") {
        tips.push(
            "Window function detected. Window operations scan the entire window partition."
                .to_string(),
        );
    }
    if tips.is_empty() {
        tips.push("Query plan looks straightforward. Execution should be efficient.".to_string());
    }
    tips
}
