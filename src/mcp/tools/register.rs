use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::security::resolve_registration_path;
use crate::utils::register_view_for_file;
use std::path::Path;
use std::sync::atomic::Ordering;
fn is_workspace_file_name(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
        && path.is_relative()
        && path.components().count() == 1
        && path.file_name().and_then(|name| name.to_str()) == Some(value)
}
pub struct RegisterDataSourceHandler;
#[async_trait::async_trait]
impl ToolHandler for RegisterDataSourceHandler {
    fn name(&self) -> &str {
        "register_data_source"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "register_data_source".to_string(),
            description: "Register a file name from the server workspace as a DuckDB view. Do not provide directories or absolute paths. Supports CSV, Parquet, Excel, GeoJSON, JSON/JSONL, Shapefile, GeoPackage. COST: 0 DB queries. USE: Before any analysis on new files. The response includes an alias (vw_...) — always use that alias for SQL queries.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {"file_path": {"type": "string"}, "format": {"type": "string", "enum": ["csv", "parquet", "excel", "geojson", "json", "shp", "gpkg"]}, "sheet_name": {"type": "string"}}, "required": ["file_path", "format"]}),
        }
    }
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let file_path = params
            .get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError::invalid_params("Missing required parameter: file_path"))?;
        if !is_workspace_file_name(file_path) {
            return Err(JsonRpcError::invalid_params(
                "file_path must be a file name in the workspace",
            ));
        }
        let format = params
            .get("format")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError::invalid_params("Missing required parameter: format"))?;
        let sheet_name = params.get("sheet_name").and_then(|v| v.as_str());
        let resolved_path = if std::path::Path::new(file_path).is_relative() {
            std::path::Path::new(&ctx.config.workspace_dir)
                .join(file_path)
                .to_string_lossy()
                .to_string()
        } else {
            file_path.to_string()
        };
        let canonical = resolve_registration_path(&resolved_path, &ctx.config.workspace_dir)
            .map_err(|e| {
                tracing::warn!("Registration path rejected: {}", e);
                JsonRpcError::invalid_params("The requested data source path is invalid")
            })?;
        if !std::path::Path::new(&canonical).exists() {
            return Err(JsonRpcError::invalid_params(
                "The requested data source was not found",
            ));
        }
        const MAX_FILE_BYTES: u64 = 500 * 1024 * 1024;
        if let Ok(meta) = std::fs::metadata(&canonical) {
            if meta.len() > MAX_FILE_BYTES {
                return Err(JsonRpcError::invalid_params(format!(
                    "File too large ({} MB). Maximum: 500 MB.",
                    meta.len() / (1024 * 1024)
                )));
            }
        }
        let info = register_view_for_file(
            std::path::Path::new(&canonical),
            std::path::Path::new(&ctx.config.state_dir),
            format,
            sheet_name,
            Some(&ctx.query_id),
            &*ctx.actor,
        )
        .await
        .map_err(|error| {
            tracing::error!("Data source registration failed: {}", error);
            JsonRpcError::internal_error("Data source registration failed")
        })?;
        ctx.metrics.queries.fetch_add(1, Ordering::Relaxed);
        let resp = serde_json::json!({
            "alias": info.alias_name.unwrap_or_else(|| String::from("N/A")),
            "format": info.format,
        });
        Ok(super::standard_response(resp, vec![]))
    }
}
