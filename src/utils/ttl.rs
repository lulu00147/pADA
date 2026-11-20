use crate::config::ServerConfig;
use crate::engine::TimedActor;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
const DATA_FILE_EXTENSIONS: &[&str] = &[
    "csv", "tsv", "parquet", "pq", "xlsx", "geojson", "gpkg", "shp", "shx", "dbf", "prj", "cpg",
    "json", "jsonl", "ndjson",
];
#[derive(Debug, Default)]
pub struct SweepReport {
    pub removed: Vec<String>,
    pub failures: Vec<String>,
}
pub fn file_ttl_secs(name: &str, default_ttl_secs: u64, underscore_ttl_secs: u64) -> u64 {
    if name.starts_with('_') {
        underscore_ttl_secs
    } else {
        default_ttl_secs
    }
}
pub fn is_expired(modified: SystemTime, ttl_secs: u64, now: SystemTime) -> bool {
    if ttl_secs == 0 {
        return false;
    }
    match now.duration_since(modified) {
        Ok(age) => age >= Duration::from_secs(ttl_secs),
        Err(_) => false,
    }
}
fn is_sweepable_data_file(name: &str) -> bool {
    if name.starts_with('.') {
        return false;
    }
    let Some(extension) = Path::new(name).extension().and_then(|e| e.to_str()) else {
        return false;
    };
    DATA_FILE_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str())
}
pub fn expired_workspace_files(
    dir: &Path,
    default_ttl_secs: u64,
    underscore_ttl_secs: u64,
    now: SystemTime,
) -> Vec<PathBuf> {
    if default_ttl_secs == 0 && underscore_ttl_secs == 0 {
        return Vec::new();
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut expired = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !is_sweepable_data_file(name) {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        let Ok(modified) = metadata.modified() else {
            continue;
        };
        let ttl_secs = file_ttl_secs(name, default_ttl_secs, underscore_ttl_secs);
        if is_expired(modified, ttl_secs, now) {
            expired.push(path);
        }
    }
    expired.sort();
    expired
}
pub async fn sweep_expired_files(
    actor: &TimedActor,
    config: &ServerConfig,
    now: SystemTime,
) -> SweepReport {
    let mut report = SweepReport::default();
    let default_ttl_secs = config.data_file_ttl_secs;
    let underscore_ttl_secs = config.underscore_data_file_ttl_secs;
    if default_ttl_secs == 0 && underscore_ttl_secs == 0 {
        return report;
    }
    let dir = PathBuf::from(&config.workspace_dir);
    let scanned = tokio::task::spawn_blocking(move || {
        expired_workspace_files(&dir, default_ttl_secs, underscore_ttl_secs, now)
    })
    .await;
    let expired = match scanned {
        Ok(files) => files,
        Err(error) => {
            report
                .failures
                .push(format!("workspace scan task failed: {error}"));
            return report;
        }
    };
    let state = Path::new(&config.state_dir);
    for path in expired {
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        match crate::utils::view_registry::unregister_and_remove_file(&path, state, actor).await {
            Ok(_) => {
                tracing::info!("TTL removed expired data file: {}", path.display());
                report.removed.push(label);
            }
            Err(error) => {
                tracing::warn!("TTL: cannot expire '{}': {}", label, error);
                report.failures.push(format!("{label}: {error}"));
            }
        }
    }
    report
}
