use crate::engine::{execute_streaming, stream_result_to_json_array};
use crate::mcp::error::JsonRpcError;
use crate::mcp::protocol::{HandlerContext, ToolDefinition, ToolHandler};
use crate::security::inspect_sql;
use crate::utils::engine_err_to_jsonrpc;
use std::sync::atomic::Ordering;
pub struct SummarizeTableHandler;
#[async_trait::async_trait]
impl ToolHandler for SummarizeTableHandler {
    fn name(&self) -> &str {
        "summarize_table"
    }
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "summarize_table".to_string(),
            description: "Complete column stats in ONE call for a registered vw_<16 hex> view: null_level, cardinality_ratio, min/max/avg, quantiles, AND recommended_aggregations. COST: 2 DB queries. USE: After discover_workspace when you need deep column insight. table_name must be a vw_ view, never a tm_ table.".to_string(),
            input_schema: serde_json::json!({"type": "object", "properties": {"table_name": {"type": "string"}}, "required": ["table_name"]}),
        }
    }
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError> {
        let table_name = params
            .get("table_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                JsonRpcError::invalid_params("Missing required parameter: table_name")
            })?;
        if table_name.trim().is_empty() {
            return Err(JsonRpcError::invalid_params(
                "table_name cannot be empty. Provide a vw_<16 hex> view name.",
            ));
        }
        let alias = table_name.trim();
        if !crate::security::is_public_alias(alias) {
            return Err(JsonRpcError::invalid_params(
                "table_name must be a registered vw_<16 hex> view",
            ));
        }
        let qualified = format!("\"{}\"", alias);
        let (count_sql, summarize_sql) = (
            format!("SELECT count(*) AS cnt FROM {qualified}"),
            format!("SUMMARIZE SELECT * FROM {qualified}"),
        );
        let count_verdict = inspect_sql(&count_sql, ctx.config, true);
        let (safe_count_sql, mut warnings) = super::unpack_verdict(&count_verdict)?;
        let count_stream = execute_streaming(
            ctx.actor,
            &safe_count_sql,
            &Default::default(),
            Some(&ctx.query_id),
        )
        .await
        .map_err(|e| engine_err_to_jsonrpc(&e))?;
        let row_count = stream_result_to_json_array(count_stream)
            .first()
            .and_then(|r| r.get("cnt"))
            .and_then(|c| {
                c.as_i64()
                    .or_else(|| c.as_str().and_then(|s| s.parse().ok()))
            })
            .unwrap_or(0);
        let summarize_verdict = inspect_sql(&summarize_sql, ctx.config, true);
        let (safe_summarize_sql, sw) = super::unpack_verdict(&summarize_verdict)?;
        warnings.extend(sw);
        let summarize_stream = execute_streaming(
            ctx.actor,
            &safe_summarize_sql,
            &Default::default(),
            Some(&ctx.query_id),
        )
        .await
        .map_err(|e| engine_err_to_jsonrpc(&e))?;
        let columns = serde_json::Value::Array(stream_result_to_json_array(summarize_stream));
        let enriched_columns: Vec<serde_json::Value> = columns
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|col| {
                        let mut enriched = col.clone();
                        let obj = match enriched.as_object_mut() {
                            Some(o) => o,
                            None => return None,
                        };
                        if let Some(np) = obj
                            .get("null_percentage")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                        {
                            let level = if np < 5.0 {
                                "low"
                            } else if np < 20.0 {
                                "medium"
                            } else if np < 50.0 {
                                "high"
                            } else {
                                "critical"
                            };
                            obj.insert("null_level".into(), serde_json::json!(level));
                        }
                        if let (Some(uniq), true) = (
                            obj.get("approx_unique")
                                .and_then(|v| v.as_str())
                                .and_then(|s| s.parse::<f64>().ok()),
                            row_count > 0,
                        ) {
                            let ratio = uniq / row_count as f64;
                            obj.insert(
                                "cardinality_ratio".into(),
                                serde_json::json!((ratio * 1000.0).round() / 1000.0),
                            );
                        }
                        Some(enriched)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let summary_text = if enriched_columns.is_empty() {
            format!(
                "Table '{}' has {} rows but SUMMARIZE returned no column data.",
                table_name, row_count
            )
        } else {
            let mut parts = vec![format!(
                "Table '{}': {} rows, {} columns.",
                table_name,
                row_count,
                enriched_columns.len()
            )];
            let high_null: Vec<_> = enriched_columns
                .iter()
                .filter(|c| {
                    c.get("null_level").and_then(|v| v.as_str()) == Some("high")
                        || c.get("null_level").and_then(|v| v.as_str()) == Some("critical")
                })
                .filter_map(|c| c.get("column_name").and_then(|v| v.as_str()))
                .collect();
            if !high_null.is_empty() {
                parts.push(format!(
                    "High-null columns (null ratio >= 20%): {}.",
                    high_null.join(", ")
                ));
            }
            let num_cols = enriched_columns
                .iter()
                .filter(|c| {
                    c.get("column_type")
                        .and_then(|v| v.as_str())
                        .map(|t| {
                            t.contains("INT")
                                || t.contains("FLOAT")
                                || t.contains("DOUBLE")
                                || t.contains("DECIMAL")
                                || t.contains("NUMERIC")
                        })
                        .unwrap_or(false)
                })
                .count();
            if num_cols > 0 {
                parts.push(format!(
                    "{} numeric columns available for aggregation.",
                    num_cols
                ));
            }
            parts.join(" ")
        };
        let mut recommended_aggregations: Vec<serde_json::Value> = Vec::new();
        for col in &enriched_columns {
            let col_name = match col.get("column_name").and_then(|v| v.as_str()) {
                Some(n) => n,
                None => continue,
            };
            let col_type = match col.get("column_type").and_then(|v| v.as_str()) {
                Some(t) => t,
                None => continue,
            };
            let null_pct = col
                .get("null_percentage")
                .and_then(|v| v.as_str())
                .and_then(|s| s.trim_end_matches('%').parse::<f64>().ok())
                .unwrap_or(0.0);
            let suggestion = if col_type.contains("INT")
                || col_type.contains("FLOAT")
                || col_type.contains("DOUBLE")
                || col_type.contains("DECIMAL")
                || col_type.contains("NUMERIC")
            {
                if null_pct >= 50.0 {
                    continue;
                }
                Some(serde_json::json!({
                    "column": col_name,
                    "type": "numeric",
                    "aggregations": ["AVG", "SUM", "MIN", "MAX"],
                    "example_sql": format!("SELECT AVG(\"{}\"), SUM(\"{}\"), MIN(\"{}\"), MAX(\"{}\") FROM \"{}\"",
                        col_name, col_name, col_name, col_name, table_name)
                }))
            } else if col_type.contains("TIMESTAMP") || col_type.contains("DATE") {
                if null_pct >= 50.0 {
                    continue;
                }
                Some(serde_json::json!({
                    "column": col_name,
                    "type": "temporal",
                    "aggregations": ["TRUNC", "COUNT", "MIN", "MAX"],
                    "example_sql": format!("SELECT DATE_TRUNC('day', \"{}\") AS day, COUNT(*) FROM \"{}\" GROUP BY day",
                        col_name, table_name)
                }))
            } else if col_type == "VARCHAR" {
                let approx_unique = col
                    .get("approx_unique")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                if approx_unique > 0.0 && approx_unique < 1000.0 && null_pct < 50.0 {
                    Some(serde_json::json!({
                        "column": col_name,
                        "type": "categorical",
                        "aggregations": ["COUNT", "GROUP BY"],
                        "example_sql": format!("SELECT \"{}\", COUNT(*) AS cnt FROM \"{}\" GROUP BY \"{}\" ORDER BY cnt DESC LIMIT 10",
                            col_name, table_name, col_name)
                    }))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(s) = suggestion {
                recommended_aggregations.push(s);
            }
        }
        let column_count = enriched_columns.len();
        ctx.metrics.queries.fetch_add(2, Ordering::Relaxed);
        Ok(super::standard_response(
            serde_json::json!({
                "table_name": table_name,
                "total_rows": row_count,
                "column_count": column_count,
                "columns": enriched_columns,
                "summary": summary_text,
                "recommended_aggregations": recommended_aggregations,
            }),
            warnings,
        ))
    }
}
