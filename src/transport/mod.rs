use crate::config::Config;
use crate::mcp::Router;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
#[cfg(feature = "http")]
pub const SESSION_TTL_SECS: u64 = 3600;
#[cfg(feature = "http")]
pub fn session_is_expired(last_seen: std::time::Instant, cutoff: std::time::Instant) -> bool {
    last_seen <= cutoff
}
#[cfg(feature = "http")]
const HTTP_SCOPE_PREFIX: &str = "http:";
pub fn origin_is_allowed(origin: &str, allowed: &str) -> bool {
    allowed == "*" || allowed.split(',').any(|a| a.trim() == origin)
}
pub fn http_bind_addr(
    host: &str,
    port: u16,
    allow_remote_http: bool,
    http_bearer_token: &str,
    allowed_origins: &str,
) -> Result<SocketAddr, String> {
    let ip = host
        .parse::<IpAddr>()
        .map_err(|_| "HTTP host must be a literal IP address".to_string())?;
    if http_bearer_token.is_empty() {
        return Err("HTTP transport requires a non-empty bearer token".to_string());
    }
    if !ip.is_loopback() {
        if !allow_remote_http {
            return Err("HTTP host must be loopback unless allow_remote_http is true".to_string());
        }
        if allowed_origins.trim() == "*" {
            tracing::warn!("allowed_origins is '*' — CORS allows any origin. Bearer token is the sole security layer.");
        }
    }
    Ok(SocketAddr::new(ip, port))
}
#[cfg(feature = "http")]
pub async fn run_http_server(
    config: Arc<Config>,
    server: Arc<Router>,
) -> Result<(), Box<dyn std::error::Error>> {
    http_impl::run_http_server_inner(config, server).await
}
#[cfg(not(feature = "http"))]
pub async fn run_http_server(
    _config: Arc<Config>,
    _server: Arc<Router>,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("HTTP transport requires the `http` feature; rebuild with `--features http`".into())
}
#[cfg(feature = "http")]
mod http_impl {
    use super::{http_bind_addr, origin_is_allowed, HTTP_SCOPE_PREFIX, SESSION_TTL_SECS};
    use crate::config::Config;
    use crate::mcp::Router as McpRouter;
    use axum::{
        extract::{DefaultBodyLimit, Multipart, Request, State},
        http::{header, HeaderValue, StatusCode},
        middleware::{self, Next},
        response::{IntoResponse, Json, Response},
        routing::{delete, get, post},
        Router,
    };
    use std::collections::HashMap;
    use std::io::Read;
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use tokio::io::AsyncWriteExt;
    use tokio::sync::RwLock;
    use tower_http::cors::CorsLayer;
    use uuid::Uuid;
    #[derive(Clone)]
    pub struct HttpState {
        pub server: Arc<McpRouter>,
        pub sessions: Arc<RwLock<HashMap<String, Instant>>>,
        pub config: Arc<Config>,
    }
    const MAX_SESSIONS: usize = 10_000;
    const MAX_ZIP_ENTRIES: usize = 500;
    const MAX_ZIP_ENTRY_BYTES: u64 = 500 * 1024 * 1024;
    const MAX_ZIP_TOTAL_BYTES: u64 = 2_000 * 1024 * 1024;
    fn bearer_token_is_authorized(headers: &axum::http::HeaderMap, expected: &str) -> bool {
        let mut values = headers.get_all(header::AUTHORIZATION).iter();
        let Some(value) = values.next() else {
            return false;
        };
        if values.next().is_some() {
            return false;
        }
        value
            .to_str()
            .ok()
            .and_then(|value| value.strip_prefix("Bearer "))
            .is_some_and(|token| token == expected)
    }
    fn unauthorized_response() -> Response {
        let mut response = StatusCode::UNAUTHORIZED.into_response();
        response
            .headers_mut()
            .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        response
    }
    async fn bearer_authentication(
        State(state): State<HttpState>,
        request: Request,
        next: Next,
    ) -> Response {
        if request.method() == axum::http::Method::OPTIONS
            || bearer_token_is_authorized(request.headers(), &state.config.server.http_bearer_token)
        {
            next.run(request).await
        } else {
            unauthorized_response()
        }
    }
    fn zip_error(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
        (status, Json(serde_json::json!({"error": message})))
    }
    fn publish_with_dedup(
        temp_path: &std::path::Path,
        dest_dir: &std::path::Path,
        safe_name: &str,
    ) -> Result<std::path::PathBuf, StatusCode> {
        let dest_path = dest_dir.join(safe_name);
        if !dest_path.exists() {
            std::fs::hard_link(temp_path, &dest_path)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let _ = std::fs::remove_file(temp_path);
            return Ok(dest_path);
        }
        let stem = std::path::Path::new(safe_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let ext = std::path::Path::new(safe_name)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{e}"))
            .unwrap_or_default();
        for i in 1u32..=999 {
            let candidate = dest_dir.join(format!("{}_{}{}", stem, i, ext));
            if !candidate.exists() {
                std::fs::hard_link(temp_path, &candidate)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                let _ = std::fs::remove_file(temp_path);
                return Ok(candidate);
            }
        }
        let _ = std::fs::remove_file(temp_path);
        Err(StatusCode::CONFLICT)
    }
    fn is_within_directory(path: &std::path::Path, directory: &std::path::Path) -> bool {
        path.starts_with(directory)
    }
    fn http_scope(session_id: &str) -> String {
        format!("{HTTP_SCOPE_PREFIX}{session_id}")
    }
    async fn refresh_session(state: &HttpState, session_id: &str) -> Result<(), StatusCode> {
        let mut sessions = state.sessions.write().await;
        let expired = match sessions.get(session_id) {
            Some(last_seen) => last_seen.elapsed() >= Duration::from_secs(SESSION_TTL_SECS),
            None => true,
        };
        if expired {
            sessions.remove(session_id);
            return Err(StatusCode::NOT_FOUND);
        }
        sessions.insert(session_id.to_owned(), Instant::now());
        Ok(())
    }
    async fn mcp_post_handler(
        State(state): State<HttpState>,
        headers: axum::http::HeaderMap,
        body: String,
    ) -> axum::response::Response {
        if let Some(origin) = headers.get("Origin") {
            let origin_str = match origin.to_str() {
                Ok(s) => s,
                Err(_) => {
                    tracing::warn!("Origin header contains invalid UTF-8, rejecting");
                    return StatusCode::FORBIDDEN.into_response();
                }
            };
            if !origin_is_allowed(origin_str, &state.config.server.allowed_origins) {
                tracing::warn!("Origin not allowed: {}", origin_str);
                return StatusCode::FORBIDDEN.into_response();
            }
        }
        if let Some(version) = headers.get("MCP-Protocol-Version") {
            match version.to_str().unwrap_or("") {
                "2024-11-05" | "2025-03-26" | "2025-06-18" | "2025-11-25" => {}
                _ => return StatusCode::BAD_REQUEST.into_response(),
            }
        }
        let session_id = headers
            .get("Mcp-Session-Id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let parsed = serde_json::from_str::<serde_json::Value>(&body).unwrap_or_else(|e| {
            tracing::warn!("transport: failed to parse request body as JSON: {}", e);
            serde_json::Value::Null
        });
        let is_initialize = parsed.get("method").and_then(|m| m.as_str()) == Some("initialize");
        if !is_initialize {
            let sid = match &session_id {
                Some(id) => match refresh_session(&state, id).await {
                    Ok(()) => id.clone(),
                    Err(status) => return status.into_response(),
                },
                None => return StatusCode::BAD_REQUEST.into_response(),
            };
            let scope = http_scope(&sid);
            let response = state
                .server
                .handle_message_scoped(&body, Some(&scope))
                .await;
            build_json_response(response, Some(sid))
        } else {
            let new_id = Uuid::new_v4().to_string();
            let mut sessions = state.sessions.write().await;
            if sessions.len() >= MAX_SESSIONS {
                if let Some(oldest) = sessions
                    .iter()
                    .min_by_key(|(_, t)| **t)
                    .map(|(k, _)| k.clone())
                {
                    sessions.remove(&oldest);
                }
            }
            sessions.insert(new_id.clone(), Instant::now());
            drop(sessions);
            let scope = http_scope(&new_id);
            let response = state
                .server
                .handle_message_scoped(&body, Some(&scope))
                .await;
            tracing::info!("MCP session created: {}", new_id);
            build_json_response(response, Some(new_id))
        }
    }
    fn build_json_response(
        response: Option<serde_json::Value>,
        session_id: Option<String>,
    ) -> axum::response::Response {
        let mut resp = match response {
            Some(json) => (StatusCode::OK, Json(json)).into_response(),
            None => StatusCode::ACCEPTED.into_response(),
        };
        resp.headers_mut().insert(
            "MCP-Protocol-Version",
            HeaderValue::from_static("2025-11-25"),
        );
        if let Some(sid) = session_id {
            resp.headers_mut().insert(
                "Mcp-Session-Id",
                HeaderValue::from_str(&sid).unwrap_or_else(|_| HeaderValue::from_static("unknown")),
            );
        }
        resp
    }
    async fn delete_handler(
        State(state): State<HttpState>,
        headers: axum::http::HeaderMap,
    ) -> StatusCode {
        let session_id = match headers.get("Mcp-Session-Id").and_then(|v| v.to_str().ok()) {
            Some(id) => id.to_string(),
            None => return StatusCode::BAD_REQUEST,
        };
        if state.sessions.write().await.remove(&session_id).is_some() {
            state.server.drop_scope(&http_scope(&session_id)).await;
            tracing::info!("MCP session terminated: {}", session_id);
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    }
    fn is_reserved_upload_name(name: &str) -> bool {
        name.eq_ignore_ascii_case(".duckdb_views.json")
            || name.eq_ignore_ascii_case(".duckdb_views.lock")
    }
    struct ExtractedFilesGuard {
        paths: Vec<std::path::PathBuf>,
        committed: bool,
    }
    impl ExtractedFilesGuard {
        fn new() -> Self {
            Self {
                paths: Vec::new(),
                committed: false,
            }
        }
        fn commit(mut self) -> Vec<String> {
            self.committed = true;
            std::mem::take(&mut self.paths)
                .into_iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect()
        }
    }
    impl Drop for ExtractedFilesGuard {
        fn drop(&mut self) {
            if !self.committed {
                for path in &self.paths {
                    let _ = std::fs::remove_file(path);
                }
            }
        }
    }
    fn extract_zip(
        zip_file: std::fs::File,
        dest_dir: &str,
    ) -> Result<Vec<String>, (StatusCode, Json<serde_json::Value>)> {
        let mut archive = zip::ZipArchive::new(zip_file)
            .map_err(|_| zip_error(StatusCode::BAD_REQUEST, "Invalid ZIP file"))?;
        if archive.len() > MAX_ZIP_ENTRIES {
            return Err(zip_error(
                StatusCode::PAYLOAD_TOO_LARGE,
                "ZIP contains too many entries",
            ));
        }
        let mut extracted = ExtractedFilesGuard::new();
        let mut total_size = 0u64;
        let dest = std::fs::canonicalize(dest_dir).map_err(|_| {
            zip_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Cannot resolve extraction directory",
            )
        })?;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error":"Cannot read ZIP entry"})),
                )
            })?;
            let Some(out_path) = entry.enclosed_name() else {
                continue;
            };
            let full_path = dest.join(&out_path);
            if full_path.is_symlink() {
                return Err(zip_error(
                    StatusCode::BAD_REQUEST,
                    "ZIP entry path may not be a symbolic link",
                ));
            }
            if entry.size() > MAX_ZIP_ENTRY_BYTES {
                return Err(zip_error(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "ZIP entry exceeds maximum extracted size",
                ));
            }
            if entry.is_dir() {
                std::fs::create_dir_all(&full_path).map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error":"Cannot create ZIP directory"})),
                    )
                })?;
                let canonical_dir = std::fs::canonicalize(&full_path).map_err(|_| {
                    zip_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Cannot resolve ZIP directory",
                    )
                })?;
                if !is_within_directory(&canonical_dir, &dest) {
                    return Err(zip_error(
                        StatusCode::BAD_REQUEST,
                        "ZIP entry attempted path escape",
                    ));
                }
                continue;
            }
            if out_path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(is_reserved_upload_name)
            {
                return Err(zip_error(
                    StatusCode::BAD_REQUEST,
                    "ZIP contains a reserved server file name",
                ));
            }
            let parent = full_path.parent().ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error":"Invalid ZIP entry path"})),
                )
            })?;
            std::fs::create_dir_all(parent).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error":"Cannot create ZIP directory"})),
                )
            })?;
            let canonical_parent = std::fs::canonicalize(parent).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error":"Cannot resolve ZIP output directory"})),
                )
            })?;
            if !is_within_directory(&canonical_parent, &dest) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error":"ZIP entry attempted path escape"})),
                ));
            }
            if full_path.exists() {
                let canonical_path = std::fs::canonicalize(&full_path).map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"error":"Cannot resolve ZIP output path"})),
                    )
                })?;
                if !is_within_directory(&canonical_path, &dest) {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"error":"ZIP entry attempted path escape"})),
                    ));
                }
            }
            let final_path = if full_path.exists() {
                let stem = full_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("file");
                let ext = full_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| format!(".{e}"))
                    .unwrap_or_default();
                let parent = full_path.parent().unwrap_or(&dest);
                let mut candidate = None;
                for i in 1u32..=999 {
                    let p = parent.join(format!("{}_{}{}", stem, i, ext));
                    if !p.exists() {
                        candidate = Some(p);
                        break;
                    }
                }
                candidate.unwrap_or(full_path.clone())
            } else {
                full_path.clone()
            };
            let mut output = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&final_path)
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error":"Cannot create extracted file"})),
                    )
                })?;
            let remaining_total = MAX_ZIP_TOTAL_BYTES.saturating_sub(total_size);
            let copy_limit = (MAX_ZIP_ENTRY_BYTES + 1).min(remaining_total + 1);
            let copied = match std::io::copy(&mut entry.by_ref().take(copy_limit), &mut output) {
                Ok(copied) => copied,
                Err(_) => {
                    let _ = std::fs::remove_file(&final_path);
                    return Err(zip_error(
                        StatusCode::BAD_REQUEST,
                        "Cannot extract ZIP entry",
                    ));
                }
            };
            let exceeds_entry_limit = copied > MAX_ZIP_ENTRY_BYTES;
            total_size = match total_size.checked_add(copied) {
                Some(size) => size,
                None => {
                    let _ = std::fs::remove_file(&final_path);
                    return Err(zip_error(
                        StatusCode::PAYLOAD_TOO_LARGE,
                        "ZIP extracted size overflow",
                    ));
                }
            };
            if exceeds_entry_limit || total_size > MAX_ZIP_TOTAL_BYTES {
                let _ = std::fs::remove_file(&final_path);
                return Err(zip_error(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    if exceeds_entry_limit {
                        "ZIP entry exceeds maximum extracted size"
                    } else {
                        "ZIP exceeds maximum total extracted size"
                    },
                ));
            }
            if output.sync_all().is_err() {
                let _ = std::fs::remove_file(&final_path);
                return Err(zip_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Cannot sync extracted file",
                ));
            }
            extracted.paths.push(final_path);
        }
        if extracted.paths.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error":"ZIP contains no files"})),
            ));
        }
        Ok(extracted.commit())
    }
    async fn auto_register_file(
        server: &McpRouter,
        state_dir: &str,
        path: &std::path::Path,
    ) -> Result<crate::utils::ViewInfo, String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let format = match ext.as_str() {
            "csv" | "tsv" => "csv",
            "parquet" | "pq" => "parquet",
            "xlsx" => "excel",
            "geojson" => "geojson",
            "shp" => "shp",
            "gpkg" => "gpkg",
            "json" | "jsonl" | "ndjson" => "json",
            _ => return Err("unsupported format".into()),
        };
        if format == "csv" {
            crate::security::csv_encoding::ensure_utf8_inplace(path)
                .map_err(|e| format!("CSV UTF-8 conversion failed: {e}"))?;
        }
        crate::utils::register_view_for_file(
            path,
            std::path::Path::new(state_dir),
            format,
            None,
            None,
            server.actor(),
        )
        .await
    }
    async fn upload_handler(
        State(state): State<HttpState>,
        headers: axum::http::HeaderMap,
        mut multipart: Multipart,
    ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
        let session_id = headers
            .get("Mcp-Session-Id")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| zip_error(StatusCode::BAD_REQUEST, "Missing MCP session ID"))?;
        refresh_session(&state, session_id)
            .await
            .map_err(|status| zip_error(status, "Unknown MCP session ID"))?;
        let dest_dir = state.config.server.workspace_dir.clone();
        std::fs::create_dir_all(&dest_dir).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error":"Cannot create workspace directory"})),
            )
        })?;
        if let Some(mut field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error":"Invalid multipart field"})),
            )
        })? {
            let file_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("upload_{}", Uuid::new_v4()));
            let safe_name = file_name.replace(
                ['/', '\\', ':', ';', '*', '?', '"', '<', '>', '|', '\0'],
                "_",
            );
            if is_reserved_upload_name(&safe_name) {
                return Err(zip_error(
                    StatusCode::BAD_REQUEST,
                    "Reserved server file name",
                ));
            }
            if std::path::Path::new(&safe_name)
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error":"Invalid file name"})),
                ));
            }
            let ext = std::path::Path::new(&safe_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            const ALLOWED_EXTENSIONS: &[&str] = &[
                "csv", "tsv", "parquet", "xlsx", "geojson", "gpkg", "shp", "zip", "json", "jsonl",
                "ndjson",
            ];
            if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
                return Err(zip_error(
                    StatusCode::BAD_REQUEST,
                    "Unsupported file extension",
                ));
            }
            let temp_path =
                std::path::Path::new(&dest_dir).join(format!(".upload-{}", Uuid::new_v4()));
            let mut temp_file = tokio::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temp_path)
                .await
                .map_err(|_| {
                    zip_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Cannot create upload file",
                    )
                })?;
            let mut size_bytes = 0u64;
            let mut signature = Vec::with_capacity(4);
            loop {
                let chunk = field.chunk().await.map_err(|_| {
                    let _ = std::fs::remove_file(&temp_path);
                    zip_error(StatusCode::BAD_REQUEST, "Cannot read field data")
                })?;
                let Some(chunk) = chunk else { break };
                size_bytes = match size_bytes.checked_add(chunk.len() as u64) {
                    Some(size) if size <= crate::mcp::tools::MAX_UPLOAD_BYTES => size,
                    _ => {
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        return Err(zip_error(
                            StatusCode::PAYLOAD_TOO_LARGE,
                            "File exceeds maximum upload size",
                        ));
                    }
                };
                let needed = 4usize.saturating_sub(signature.len());
                signature.extend_from_slice(&chunk[..chunk.len().min(needed)]);
                if temp_file.write_all(&chunk).await.is_err() {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return Err(zip_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Cannot write file",
                    ));
                }
            }
            if temp_file.sync_all().await.is_err() {
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(zip_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Cannot sync upload file",
                ));
            }
            if ext == "zip" && signature == [0x50, 0x4B, 0x03, 0x04] {
                let dest = dest_dir.clone();
                let temp_path_clone = temp_path.clone();
                drop(temp_file);
                let extracted = tokio::task::spawn_blocking(move || {
                    let file = std::fs::File::open(&temp_path_clone).map_err(|_| {
                        zip_error(StatusCode::BAD_REQUEST, "Cannot open uploaded ZIP")
                    })?;
                    extract_zip(file, &dest)
                })
                .await
                .map_err(|_| {
                    zip_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "ZIP extraction task failed",
                    )
                })?;
                let _ = tokio::fs::remove_file(&temp_path).await;
                let extracted = extracted?;
                let mut registered = Vec::new();
                let mut failures = Vec::new();
                for path_str in &extracted {
                    let p = std::path::Path::new(path_str);
                    let file_name = p
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("uploaded_file");
                    match auto_register_file(&state.server, &state.config.server.state_dir, p).await
                    {
                        Ok(info) => registered.push(serde_json::json!({
                            "file": file_name,
                            "alias": info.alias_name,
                            "format": info.format,
                            "success": true,
                        })),
                        Err(error) => {
                            tracing::error!("ZIP registration failed for {}: {}", file_name, error);
                            failures.push(serde_json::json!({
                                "file": file_name,
                                "success": false,
                                "error": "File registration failed",
                            }));
                        }
                    }
                }
                if !failures.is_empty() {
                    tracing::error!(
                        "ZIP registration failed: {}",
                        serde_json::to_string(&failures).unwrap_or_default()
                    );
                    return Err(zip_error(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "ZIP file registration failed",
                    ));
                }
                return Ok(Json(serde_json::json!({
                    "success": true,
                    "extracted_files": extracted.iter().filter_map(|path| {
                        std::path::Path::new(path)
                            .file_name()
                            .and_then(|name| name.to_str())
                    }).collect::<Vec<_>>(),
                    "file_count": extracted.len(),
                    "registered_views": registered,
                })));
            }
            drop(temp_file);
            let dest_path =
                publish_with_dedup(&temp_path, std::path::Path::new(&dest_dir), &safe_name)
                    .map_err(|status| zip_error(status, "Cannot publish file"))?;
            let registered =
                auto_register_file(&state.server, &state.config.server.state_dir, &dest_path).await;
            let info = registered.map_err(|error| {
                tracing::error!("Uploaded file registration failed: {}", error);
                let _ = std::fs::remove_file(&dest_path);
                zip_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "File uploaded but registration failed",
                )
            })?;
            return Ok(Json(serde_json::json!({
                "file_name": safe_name,
                "size_bytes": size_bytes,
                "registered": true,
                "alias": info.alias_name,
                "format": info.format,
            })));
        }
        Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error":"No file uploaded"})),
        ))
    }
    async fn health_handler() -> Json<serde_json::Value> {
        Json(serde_json::json!({"status": "ok", "service": "pADA"}))
    }
    fn build_http_router(state: HttpState) -> Router {
        let allow_origin = if state.config.server.allowed_origins == "*" {
            tower_http::cors::AllowOrigin::any()
        } else {
            let origins: Vec<axum::http::HeaderValue> = state
                .config
                .server
                .allowed_origins
                .split(',')
                .filter_map(|o| match o.trim().parse() {
                    Ok(hv) => Some(hv),
                    Err(_) => {
                        tracing::warn!(
                            "Invalid origin '{}' in allowed_origins, skipping",
                            o.trim()
                        );
                        None
                    }
                })
                .collect();
            tower_http::cors::AllowOrigin::list(origins)
        };
        let cors = CorsLayer::new()
            .allow_origin(allow_origin)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::HeaderName::from_static("mcp-protocol-version"),
                axum::http::HeaderName::from_static("mcp-session-id"),
            ])
            .expose_headers([
                axum::http::HeaderName::from_static("mcp-session-id"),
                axum::http::HeaderName::from_static("mcp-protocol-version"),
            ]);
        let protected = Router::new()
            .route("/", delete(delete_handler).post(mcp_post_handler))
            .route("/upload", post(upload_handler))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                bearer_authentication,
            ));
        Router::new()
            .merge(protected)
            .route("/health", get(health_handler))
            .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
            .layer(cors)
            .with_state(state)
    }
    pub async fn run_http_server_inner(
        config: Arc<Config>,
        server: Arc<McpRouter>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = HttpState {
            server,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: config.clone(),
        };
        let addr = http_bind_addr(
            &config.server.host,
            config.server.port,
            config.server.allow_remote_http,
            &config.server.http_bearer_token,
            &config.server.allowed_origins,
        )
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;
        let app = build_http_router(state.clone());
        tracing::info!("HTTP server starting on {} (MCP API)", addr);
        let gc_sessions = state.sessions.clone();
        let gc_server = state.server.clone();
        let gc_duration = if config.server.gc_interval_secs == 0 {
            Duration::from_secs(600)
        } else {
            Duration::from_secs(config.server.gc_interval_secs)
        };
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(gc_duration);
            loop {
                interval.tick().await;
                let mut map = gc_sessions.write().await;
                let cutoff = Instant::now() - Duration::from_secs(SESSION_TTL_SECS);
                let expired: Vec<String> = map
                    .iter()
                    .filter(|(_, last_seen)| super::session_is_expired(**last_seen, cutoff))
                    .map(|(id, _)| id.clone())
                    .collect();
                map.retain(|_, last_seen| !super::session_is_expired(*last_seen, cutoff));
                drop(map);
                for session_id in &expired {
                    gc_server.drop_scope(&http_scope(session_id)).await;
                }
                let removed = expired.len();
                if removed > 0 {
                    tracing::info!("Session GC: removed {} stale session(s)", removed);
                }
            }
        });
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
