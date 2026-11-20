mod cache_policy;
mod llm_error;
pub use cache_policy::is_cacheable;
pub use llm_error::engine_err_to_jsonrpc;
pub use llm_error::interpret_duckdb_error;
mod query_cache;
pub mod sql_normalize;
pub mod ttl;
mod view_registry;
pub use query_cache::QueryCache;
pub use sql_normalize::escape_ident;
pub use sql_normalize::escape_string_literal;
pub use sql_normalize::normalize;
pub use view_registry::{
    dataset_identity, persisted_view_entry, register_view_for_file, unregister_and_remove_file,
    view_name_for_file, ViewInfo,
};
pub fn views_state_path(state_dir: &std::path::Path) -> std::path::PathBuf {
    state_dir.join(".duckdb_views.json")
}
pub fn load_persisted_views(state_dir: &std::path::Path) -> Result<Vec<serde_json::Value>, String> {
    let path = views_state_path(state_dir);
    match std::fs::read_to_string(&path) {
        Ok(content) if content.is_empty() => Ok(Vec::new()),
        Ok(content) => serde_json::from_str(&content).map_err(|e| {
            format!(
                "Failed to parse view persistence file '{}': {e}",
                path.display()
            )
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(format!(
            "Failed to read view persistence file '{}': {e}",
            path.display()
        )),
    }
}
