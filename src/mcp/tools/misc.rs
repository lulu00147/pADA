use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use std::sync::atomic::Ordering;
pub struct GetServerMetricsHandler;
#[async_trait::async_trait]
impl ToolHandler for GetServerMetricsHandler {
    fn name(&self) -> &str {
        "get_server_metrics"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "get_server_metrics".to_string(),
            description: "Server health and bounded runtime metrics. COST: 0 DB queries. USE: Debugging server availability.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
        }
    }
    async fn handle(
        &self,
        _params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        Ok(super::standard_response(
            serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "uptime_seconds": ctx.metrics.start_time.elapsed().as_secs(),
                "active_connections": 1,
                "queries_executed": ctx.metrics.queries.load(Ordering::Relaxed),
                "stream_limit": ctx.config.stream_limit,
                "query_timeout_secs": ctx.config.query_timeout_secs,
                "spatial_query_timeout_secs": ctx.config.spatial_query_timeout_secs,
            }),
            vec![],
        ))
    }
}
