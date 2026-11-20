use crate::engine::TimedActor;
use std::path::Path;
fn replace_registry_file(temp: &Path, target: &Path) -> Result<(), String> {
    std::fs::rename(temp, target).map_err(|e| format!("Cannot install view registry: {e}"))
}
fn write_registry(state: &Path, entries: &[serde_json::Value]) -> Result<(), String> {
    let json =
        serde_json::to_vec(entries).map_err(|e| format!("Cannot serialize view registry: {e}"))?;
    let target = crate::utils::views_state_path(state);
    let temp = target.with_extension(format!("json.tmp-{}", std::process::id()));
    let result = (|| -> Result<(), String> {
        let mut file = std::fs::File::create(&temp)
            .map_err(|e| format!("Cannot create registry temp file: {e}"))?;
        use std::io::Write;
        file.write_all(&json)
            .map_err(|e| format!("Cannot write view registry: {e}"))?;
        file.sync_all()
            .map_err(|e| format!("Cannot sync view registry: {e}"))?;
        replace_registry_file(&temp, &target)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}
fn partition_by_file(
    entries: Vec<serde_json::Value>,
    file_path: &str,
) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
    entries.into_iter().partition(|entry| {
        entry.get("file_path").and_then(|value| value.as_str()) == Some(file_path)
    })
}
async fn restore_registration(
    actor: &TimedActor,
    previous: Option<&serde_json::Value>,
    new_view: &str,
    new_alias: &str,
    query: Option<&str>,
) -> Vec<String> {
    let mut errors = Vec::new();
    let restore =
        |errors: &mut Vec<String>, result: Result<(), crate::engine::EngineError>, label: &str| {
            if let Err(error) = result {
                errors.push(format!("failed to restore {label}: {error}"));
            }
        };
    if let Some(entry) = previous {
        if let Some(sql) = entry.get("sql").and_then(|v| v.as_str()) {
            let view = entry
                .get("view_name")
                .and_then(|v| v.as_str())
                .unwrap_or(new_view);
            restore(
                errors.as_mut(),
                actor.register_view(view, sql, query).await,
                "source view",
            );
        }
        if let Some(sql) = entry.get("alias_sql").and_then(|v| v.as_str()) {
            let alias = entry
                .get("alias_name")
                .and_then(|v| v.as_str())
                .unwrap_or(new_alias);
            restore(
                errors.as_mut(),
                actor.register_view(alias, sql, query).await,
                "alias view",
            );
        }
        if entry.get("view_name").and_then(|v| v.as_str()) != Some(new_view) {
            restore(
                errors.as_mut(),
                actor
                    .register_view(
                        "",
                        &format!(
                            "DROP VIEW IF EXISTS {}",
                            crate::utils::escape_ident(new_view)
                        ),
                        query,
                    )
                    .await,
                "new source cleanup",
            );
        }
        if entry.get("alias_name").and_then(|v| v.as_str()) != Some(new_alias) {
            restore(
                errors.as_mut(),
                actor
                    .register_view(
                        "",
                        &format!(
                            "DROP VIEW IF EXISTS {}",
                            crate::utils::escape_ident(new_alias)
                        ),
                        query,
                    )
                    .await,
                "new alias cleanup",
            );
        }
    } else {
        restore(
            errors.as_mut(),
            actor
                .register_view(
                    "",
                    &format!(
                        "DROP VIEW IF EXISTS {}",
                        crate::utils::escape_ident(new_view)
                    ),
                    query,
                )
                .await,
            "source cleanup",
        );
        restore(
            errors.as_mut(),
            actor
                .register_view(
                    "",
                    &format!(
                        "DROP VIEW IF EXISTS {}",
                        crate::utils::escape_ident(new_alias)
                    ),
                    query,
                )
                .await,
            "alias cleanup",
        );
    }
    errors
}
static REGISTRY_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));
fn ensure_state_dir(state: &Path) -> Result<(), String> {
    std::fs::create_dir_all(state)
        .map_err(|e| format!("Cannot create state directory '{}': {e}", state.display()))
}
pub fn dataset_identity(path: &Path, format: &str, sheet_name: Option<&str>) -> String {
    format!(
        "{}|{}|{}",
        path.to_string_lossy(),
        format.to_ascii_lowercase(),
        sheet_name.unwrap_or_default()
    )
}
#[derive(serde::Serialize)]
pub struct ViewInfo {
    pub schema: String,
    pub view_name: String,
    pub alias_name: Option<String>,
    pub create_sql: String,
    pub file_path: String,
    pub source_path: String,
    pub snapshot_path: String,
    pub format: String,
    pub sheet_name: Option<String>,
    pub dataset_identity: String,
}
pub async fn register_view_for_file(
    path: &Path,
    state: &Path,
    format: &str,
    sheet: Option<&str>,
    query: Option<&str>,
    actor: &TimedActor,
) -> Result<ViewInfo, String> {
    let canonical_path = dunce::canonicalize(path)
        .map_err(|e| format!("Cannot canonicalize dataset path '{}': {e}", path.display()))?;
    let file_path = canonical_path.to_string_lossy().to_string();
    let identity = dataset_identity(&canonical_path, format, sheet);
    let source_view_name = crate::engine::generate_source_view_name(&identity);
    let alias_name = crate::engine::generate_short_alias(&identity);
    let alias_sql = crate::engine::build_alias_view_sql_for(&alias_name, &source_view_name);
    let _lock = REGISTRY_LOCK.lock().await;
    ensure_state_dir(state)?;
    let mut entries = crate::utils::load_persisted_views(state)?;
    let previous_entry = entries
        .iter()
        .find(|entry| {
            entry.get("dataset_identity").and_then(|v| v.as_str()) == Some(identity.as_str())
        })
        .cloned();
    if let Some(conflict) = entries.iter().find(|entry| {
        entry.get("alias_name").and_then(|v| v.as_str()) == Some(alias_name.as_str())
            && entry.get("dataset_identity").and_then(|v| v.as_str()) != Some(identity.as_str())
    }) {
        return Err(format!(
            "Alias '{}' is already registered for dataset {:?}",
            alias_name,
            conflict.get("dataset_identity")
        ));
    }
    let sql = crate::engine::build_create_view_sql(&source_view_name, &file_path, format, sheet)
        .map_err(|e| e.to_string())?;
    if let Err(error) = actor.register_view(&source_view_name, &sql, query).await {
        let errors = restore_registration(
            actor,
            previous_entry.as_ref(),
            &source_view_name,
            &alias_name,
            query,
        )
        .await;
        return Err(if errors.is_empty() {
            format!("register_view failed: {error}")
        } else {
            format!(
                "register_view failed: {error}; rollback failed: {}",
                errors.join("; ")
            )
        });
    }
    if let Err(e) = actor.register_view(&alias_name, &alias_sql, query).await {
        let errors = restore_registration(
            actor,
            previous_entry.as_ref(),
            &source_view_name,
            &alias_name,
            query,
        )
        .await;
        return Err(if errors.is_empty() {
            format!("alias registration failed: {e}")
        } else {
            format!(
                "alias registration failed: {e}; rollback failed: {}",
                errors.join("; ")
            )
        });
    }
    let info = ViewInfo {
        schema: "ai_scratchpad".into(),
        view_name: source_view_name,
        alias_name: Some(alias_name),
        create_sql: sql,
        file_path: file_path.clone(),
        source_path: file_path.clone(),
        snapshot_path: file_path,
        format: format.into(),
        sheet_name: sheet.map(str::to_owned),
        dataset_identity: identity,
    };
    let entry = persisted_view_entry(&info, "manual", "active");
    if let Some(pos) = entries.iter().position(|e| {
        e.get("dataset_identity").and_then(|v| v.as_str()) == Some(info.dataset_identity.as_str())
    }) {
        entries[pos] = entry;
    } else {
        entries.push(entry);
    }
    if let Err(e) = write_registry(state, &entries) {
        let alias = info.alias_name.as_deref().unwrap_or("");
        let errors = restore_registration(
            actor,
            previous_entry.as_ref(),
            &info.view_name,
            alias,
            query,
        )
        .await;
        return Err(if errors.is_empty() {
            e
        } else {
            format!("{e}; rollback failed: {}", errors.join("; "))
        });
    }
    Ok(info)
}
pub async fn unregister_and_remove_file(
    path: &Path,
    state: &Path,
    actor: &TimedActor,
) -> Result<Vec<String>, String> {
    let canonical = dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let file_path = canonical.to_string_lossy().to_string();
    let _lock = REGISTRY_LOCK.lock().await;
    let entries = crate::utils::load_persisted_views(state)?;
    let (removed, kept) = partition_by_file(entries, &file_path);
    let mut dropped = Vec::new();
    for entry in &removed {
        for key in ["alias_name", "view_name"] {
            let Some(name) = entry.get(key).and_then(|value| value.as_str()) else {
                continue;
            };
            let sql = format!("DROP VIEW IF EXISTS {};", crate::utils::escape_ident(name));
            actor
                .register_view(name, &sql, None)
                .await
                .map_err(|e| format!("failed to drop '{name}': {e}"))?;
            dropped.push(name.to_string());
        }
    }
    if !removed.is_empty() {
        write_registry(state, &kept)?;
    }
    std::fs::remove_file(path).map_err(|e| format!("cannot delete '{}': {e}", path.display()))?;
    Ok(dropped)
}
pub fn persisted_view_entry(info: &ViewInfo, origin: &str, status: &str) -> serde_json::Value {
    serde_json::json!({
        "schema": info.schema,
        "view_name": info.view_name,
        "sql": info.create_sql,
        "alias_name": info.alias_name,
        "alias_sql": info.alias_name.as_ref().map(|alias| crate::engine::build_alias_view_sql_for(alias, &info.view_name)),
        "file_path": info.file_path,
        "format": info.format,
        "sheet_name": info.sheet_name,
        "dataset_identity": info.dataset_identity,
        "registration_origin": origin,
        "state": status,
    })
}
pub fn view_name_for_file(path: &Path, workspace_dir: &str) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("auto_register");
    if let Ok(rel) = path.strip_prefix(workspace_dir) {
        let mut components: Vec<String> = rel
            .parent()
            .iter()
            .flat_map(|p| p.components())
            .filter_map(|c| c.as_os_str().to_str())
            .map(|s| s.replace(['\\', '/'], "_"))
            .filter(|s| !s.is_empty())
            .collect();
        components.push(stem.to_string());
        let base = components.join("_");
        let hash: u64 = rel
            .to_string_lossy()
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(*b as u64));
        format!("{}_{:08x}", base, hash)
    } else {
        stem.to_string()
    }
}
