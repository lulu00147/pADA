use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
impl JsonRpcError {
    pub fn parse_error() -> Self {
        Self {
            code: -32700,
            message: "Parse error".into(),
            data: None,
        }
    }
    pub fn invalid_request() -> Self {
        Self {
            code: -32600,
            message: "Invalid Request".into(),
            data: None,
        }
    }
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: -32601,
            message: "Method not found".into(),
            data: Some(serde_json::json!({"method": method})),
        }
    }
    pub fn invalid_params(detail: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: "Invalid params".into(),
            data: Some(serde_json::json!({"detail": detail.into()})),
        }
    }
    pub fn internal_error(detail: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: "Internal error".into(),
            data: Some(serde_json::json!({"detail": detail.into()})),
        }
    }
}
