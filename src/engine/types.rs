use crate::engine::EngineError;
use tokio::sync::mpsc;
pub const RESULT_BATCH_ROWS: usize = 500;
pub struct StructuredResultStream {
    pub columns: Vec<String>,
    pub batches: mpsc::Receiver<Result<Vec<Vec<serde_json::Value>>, EngineError>>,
    pub operation_id: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationState {
    Queued,
    Running,
    InterruptRequested,
    Finished,
}
