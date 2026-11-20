use thiserror::Error;
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("DuckDB error: {0}")]
    DuckDb(String),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("Engine unavailable: {0}")]
    Shutdown(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
impl EngineError {
    pub fn duckdb(msg: impl Into<String>) -> Self {
        Self::DuckDb(msg.into())
    }
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }
    pub fn shutdown(msg: impl Into<String>) -> Self {
        Self::Shutdown(msg.into())
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
impl From<serde_json::Error> for EngineError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(format!("JSON error: {}", e))
    }
}
