use crate::engine::types::RESULT_BATCH_ROWS;
use crate::engine::EngineError;
use crate::engine::TimedActor;
pub fn truncate_cell(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_chars).collect();
        format!(
            "{}... [TRUNCATED, total length: {} bytes]",
            truncated,
            text.len()
        )
    }
}
fn truncate_cells(
    raw_rows: Vec<Vec<serde_json::Value>>,
    options: &StreamOptions,
) -> Vec<Vec<serde_json::Value>> {
    raw_rows
        .into_iter()
        .take(options.max_rows)
        .map(|row| {
            row.into_iter()
                .map(|cell| match cell {
                    serde_json::Value::String(s) => {
                        serde_json::Value::String(truncate_cell(&s, options.max_cell_chars))
                    }
                    serde_json::Value::Number(n) => serde_json::Value::Number(n),
                    serde_json::Value::Bool(b) => serde_json::Value::Bool(b),
                    serde_json::Value::Null => serde_json::Value::Null,
                    other => serde_json::Value::String(truncate_cell(
                        &serde_json::to_string(&other)
                            .unwrap_or_else(|_| "<serialization error>".to_string()),
                        options.max_cell_chars,
                    )),
                })
                .collect()
        })
        .collect()
}
#[derive(Debug, Clone)]
pub struct StreamOptions {
    pub max_rows: usize,
    pub max_cell_chars: usize,
    pub max_columns: usize,
    pub max_result_bytes: usize,
}
impl Default for StreamOptions {
    fn default() -> Self {
        Self {
            max_rows: 500,
            max_cell_chars: 2000,
            max_columns: 50,
            max_result_bytes: 10 * 1024 * 1024,
        }
    }
}
#[derive(Debug, serde::Serialize)]
pub struct StreamResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub row_count: usize,
    pub truncated: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub columns_truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_fetched: Option<u64>,
}
pub fn stream_result_to_json_array(result: StreamResult) -> Vec<serde_json::Value> {
    result
        .rows
        .into_iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            for (i, col) in result.columns.iter().enumerate() {
                obj.insert(
                    col.clone(),
                    row.get(i).cloned().unwrap_or(serde_json::Value::Null),
                );
            }
            serde_json::Value::Object(obj)
        })
        .collect()
}
pub async fn execute_streaming(
    actor: &TimedActor,
    sql: &str,
    options: &StreamOptions,
    query_id: Option<&str>,
) -> Result<StreamResult, EngineError> {
    let stream = actor.read_structured_stream(sql, query_id).await?;
    consume_streaming(actor, stream, options).await
}
pub async fn consume_streaming(
    actor: &TimedActor,
    mut stream: crate::engine::types::StructuredResultStream,
    options: &StreamOptions,
) -> Result<StreamResult, EngineError> {
    if stream.columns.len() > options.max_columns {
        if let Some(operation_id) = stream.operation_id.as_deref() {
            actor.interrupt(operation_id);
        }
        return Err(EngineError::internal(format!(
            "Result has {} columns, exceeding the maximum of {}.",
            stream.columns.len(),
            options.max_columns
        )));
    }
    let mut rows = Vec::with_capacity(options.max_rows.min(RESULT_BATCH_ROWS));
    let mut result_bytes = 0usize;
    let mut truncated = false;
    while let Some(batch) = stream.batches.recv().await {
        let batch = match batch {
            Ok(batch) => batch,
            Err(error) => {
                if let Some(operation_id) = stream.operation_id.as_deref() {
                    actor.interrupt(operation_id);
                }
                return Err(error);
            }
        };
        if batch.len() > RESULT_BATCH_ROWS {
            if let Some(ref operation_id) = stream.operation_id {
                actor.interrupt(operation_id);
            }
            return Err(EngineError::internal(
                "Structured result batch exceeds 500 rows",
            ));
        }
        let remaining = options.max_rows.saturating_sub(rows.len());
        if batch.len() > remaining {
            truncated = true;
        }
        for row in batch.into_iter().take(remaining) {
            let row_bytes = serde_json::to_vec(&row)
                .map_err(|e| EngineError::internal(format!("Result serialization error: {e}")))?
                .len();
            if result_bytes.saturating_add(row_bytes) > options.max_result_bytes {
                if let Some(operation_id) = stream.operation_id.as_deref() {
                    actor.interrupt(operation_id);
                }
                return Err(EngineError::internal(format!(
                    "Query result exceeds the {} byte safety limit",
                    options.max_result_bytes
                )));
            }
            result_bytes += row_bytes;
            rows.push(row);
        }
        if rows.len() == options.max_rows {
            if let Some(operation_id) = stream.operation_id.as_deref() {
                actor.interrupt(operation_id);
            }
            break;
        }
    }
    let row_count = rows.len();
    Ok(StreamResult {
        columns: stream.columns,
        rows: truncate_cells(rows, options),
        row_count,
        truncated,
        columns_truncated: false,
        pages_fetched: None,
    })
}
