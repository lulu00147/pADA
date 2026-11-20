use crate::config::ServerConfig;
use crate::engine::TimedActor;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
fn request_query_id(id: &serde_json::Value) -> Option<String> {
    match id {
        serde_json::Value::String(value) if !value.is_empty() => Some(value.clone()),
        serde_json::Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}
fn next_query_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
use super::error::JsonRpcError;
fn rpc_response(id: serde_json::Value, result: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"jsonrpc": "2.0", "id": id, "result": result})
}
fn rpc_error(id: serde_json::Value, error: JsonRpcError) -> serde_json::Value {
    serde_json::json!({"jsonrpc": "2.0", "id": id, "error": error})
}
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    fn name(&self) -> &str;
    fn definition(&self) -> ToolDefinition;
    async fn handle(
        &self,
        params: serde_json::Value,
        ctx: &HandlerContext<'_>,
    ) -> Result<serde_json::Value, JsonRpcError>;
}
pub struct HandlerContext<'a> {
    pub config: &'a ServerConfig,
    pub actor: &'a TimedActor,
    pub metrics: &'a super::tools::Metrics,
    pub query_id: String,
}
#[derive(Debug, serde::Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}
#[async_trait::async_trait]
pub trait ResourceHandler: Send + Sync {
    fn scheme(&self) -> &str;
    async fn handle(
        &self,
        uri: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, JsonRpcError>;
    async fn list(&self) -> Vec<serde_json::Value>;
}
pub struct Router {
    config: Arc<ServerConfig>,
    actor: Arc<TimedActor>,
    metrics: Arc<super::tools::Metrics>,
    tools: Arc<Mutex<HashMap<String, Arc<dyn ToolHandler>>>>,
    resources: Arc<Mutex<Vec<Box<dyn ResourceHandler>>>>,
    initialized_scopes: Arc<Mutex<HashSet<String>>>,
    active_queries: Arc<Mutex<HashMap<String, String>>>,
}
impl Router {
    pub fn new(config: Arc<ServerConfig>, actor: Arc<TimedActor>) -> Self {
        Self {
            config,
            actor,
            metrics: Arc::new(super::tools::Metrics::new()),
            tools: Arc::new(Mutex::new(HashMap::new())),
            resources: Arc::new(Mutex::new(Vec::new())),
            initialized_scopes: Arc::new(Mutex::new(HashSet::new())),
            active_queries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn register_tool(&self, handler: Box<dyn ToolHandler>) {
        let name = handler.name().to_string();
        self.tools.lock().await.insert(name, Arc::from(handler));
    }
    pub async fn register_resource(&self, handler: Box<dyn ResourceHandler>) {
        self.resources.lock().await.push(handler);
    }
    pub fn actor(&self) -> &Arc<TimedActor> {
        &self.actor
    }
    pub async fn handle_message_scoped(
        &self,
        line: &str,
        scope: Option<&str>,
    ) -> Option<serde_json::Value> {
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_e) => {
                return Some(rpc_error(
                    serde_json::Value::Null,
                    JsonRpcError::parse_error(),
                ))
            }
        };
        let has_id = v.get("id").is_some();
        let id = v.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = match v.get("method").and_then(|m| m.as_str()) {
            Some(m) => m,
            None => return Some(rpc_error(id.clone(), JsonRpcError::invalid_request())),
        };
        let params = v.get("params").cloned().unwrap_or(serde_json::Value::Null);
        let is_notification = !has_id;
        if v.get("jsonrpc").and_then(|j| j.as_str()) != Some("2.0") {
            let err = rpc_error(id.clone(), JsonRpcError::invalid_request());
            return if is_notification { None } else { Some(err) };
        }
        let scope_key = scope.unwrap_or("stdio");
        let initialized = self.initialized_scopes.lock().await.contains(scope_key);
        if !initialized && !matches!(method, "initialize" | "ping" | "notifications/initialized") {
            let err = rpc_error(
                id.clone(),
                JsonRpcError {
                    code: -32000,
                    message: "Server not initialized".into(),
                    data: None,
                },
            );
            return if is_notification { None } else { Some(err) };
        }
        let result = match method {
            "initialize" => {
                let protocol_version = params
                    .get("protocolVersion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("2025-11-25");
                let negotiated_version = match protocol_version {
                    "2024-11-05" | "2025-03-26" | "2025-06-18" | "2025-11-25" => protocol_version,
                    _ => "2025-11-25",
                };
                Ok(
                    serde_json::json!({"protocolVersion": negotiated_version, "capabilities": {"tools": {}, "resources": {}}, "serverInfo": {"name": "pADA", "title": "pADA Server", "version": env!("CARGO_PKG_VERSION")}}),
                )
            }
            "ping" => Ok(serde_json::json!({})),
            "tools/list" => {
                let tools = self.tools.lock().await;
                let mut definitions: Vec<ToolDefinition> =
                    tools.values().map(|h| h.definition()).collect();
                definitions.sort_by(|a, b| a.name.cmp(&b.name));
                drop(tools);
                let tools_list: Vec<serde_json::Value> = definitions.into_iter().map(|t| serde_json::json!({"name": t.name, "description": t.description, "inputSchema": t.input_schema})).collect();
                Ok(serde_json::json!({"tools": tools_list}))
            }
            "tools/call" => {
                let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let mut tool_args = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                if !tool_args.is_object() {
                    let error = rpc_error(
                        id,
                        JsonRpcError::invalid_params("arguments must be an object"),
                    );
                    return if is_notification { None } else { Some(error) };
                }
                let query_id = {
                    let request_id = request_query_id(&id).unwrap_or_else(next_query_id);
                    scope
                        .map(|scope| format!("{scope}:{request_id}"))
                        .unwrap_or(request_id)
                };
                tool_args["query_id"] = serde_json::Value::String(query_id.clone());
                self.active_queries
                    .lock()
                    .await
                    .insert(query_id.clone(), scope_key.to_string());
                let handler = self.tools.lock().await.get(tool_name).cloned();
                let ctx = HandlerContext {
                    config: &self.config,
                    actor: &self.actor,
                    metrics: &self.metrics,
                    query_id: query_id.clone(),
                };
                let (content, is_error) = match handler {
                    Some(handler) => match handler.handle(tool_args, &ctx).await {
                        Ok(mut value) => {
                            if let Some(object) = value.as_object_mut() {
                                object
                                    .entry("query_id".to_string())
                                    .or_insert_with(|| serde_json::Value::String(query_id.clone()));
                            }
                            match serde_json::to_string(&value) {
                                Ok(s) => (s, false),
                                Err(e) => {
                                    let msg = format!(
                                        "Internal error: failed to serialize tool response: {}",
                                        e
                                    );
                                    tracing::error!("{}", msg);
                                    (msg, true)
                                }
                            }
                        }
                        Err(e) => {
                            let detail = e
                                .data
                                .as_ref()
                                .and_then(|d| d.get("detail"))
                                .and_then(|d| d.as_str())
                                .unwrap_or(&e.message)
                                .to_string();
                            let content = if detail == e.message {
                                e.message.clone()
                            } else {
                                format!("{}: {}", e.message, detail)
                            };
                            (content, true)
                        }
                    },
                    None => (format!("Tool not found: {}", tool_name), true),
                };
                self.active_queries.lock().await.remove(&query_id);
                Ok(
                    serde_json::json!({"content": [{"type": "text", "text": content}], "isError": is_error, "query_id": query_id}),
                )
            }
            "resources/list" => {
                let resources_lock = self.resources.lock().await;
                let mut all_resources: Vec<serde_json::Value> = Vec::new();
                for handler in resources_lock.iter() {
                    all_resources.extend(handler.list().await);
                }
                Ok(serde_json::json!({"resources": all_resources}))
            }
            "resources/read" => {
                let uri = params
                    .clone()
                    .get("uri")
                    .and_then(|u| u.as_str())
                    .unwrap_or("")
                    .to_string();
                let resources_lock = self.resources.lock().await;
                let matched = resources_lock.iter().find(|h| uri.starts_with(h.scheme()));
                match matched {
                    Some(handler) => handler.handle(&uri, params).await,
                    None => Err(JsonRpcError::invalid_params(format!(
                        "Unknown resource URI: {}",
                        uri
                    ))),
                }
            }
            "notifications/initialized" => {
                self.initialized_scopes
                    .lock()
                    .await
                    .insert(scope_key.to_string());
                tracing::info!("MCP client initialized for scope {}", scope_key);
                return None;
            }
            "notifications/cancelled" => {
                let request_id = params
                    .get("requestId")
                    .or_else(|| params.get("request_id"))
                    .and_then(request_query_id);
                match request_id {
                    Some(request_id) => {
                        let query_id = scope
                            .map(|scope| format!("{scope}:{request_id}"))
                            .unwrap_or(request_id);
                        let active = self
                            .active_queries
                            .lock()
                            .await
                            .remove(&query_id)
                            .is_some_and(|owner| owner == scope_key);
                        if active && self.actor.interrupt(&query_id) {
                            tracing::info!("Client cancelled request {}", query_id);
                        } else {
                            tracing::warn!(
                                "Client cancellation did not match an active request: {}",
                                query_id
                            );
                        }
                    }
                    None => tracing::warn!("Client cancelled request without a valid requestId"),
                }
                return None;
            }
            other => Err(JsonRpcError::method_not_found(other)),
        };
        let response = match result {
            Ok(res) => rpc_response(id, res),
            Err(err) => rpc_error(id, err),
        };
        if is_notification {
            None
        } else {
            Some(response)
        }
    }
    pub async fn drop_scope(&self, scope: &str) {
        self.initialized_scopes.lock().await.remove(scope);
        let query_ids: Vec<String> = {
            let mut active = self.active_queries.lock().await;
            let ids = active
                .iter()
                .filter(|(_, owner)| owner.as_str() == scope)
                .map(|(id, _)| id.clone())
                .collect();
            active.retain(|_, owner| owner.as_str() != scope);
            ids
        };
        for query_id in query_ids {
            self.actor.interrupt(&query_id);
        }
    }
    pub async fn handle_message(&self, line: &str) -> Option<serde_json::Value> {
        self.handle_message_scoped(line, None).await
    }
}
