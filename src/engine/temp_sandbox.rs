use std::fs;
use std::path::{Path, PathBuf};
pub struct TempSandbox {
    temp_dir: PathBuf,
    instance_id: String,
    lease_path: PathBuf,
    lease: std::fs::File,
}
impl TempSandbox {
    pub fn new(temp_dir: &Path, _workspace_dir: &str) -> Self {
        let path = temp_dir.to_path_buf();
        if let Err(e) = fs::create_dir_all(&path) {
            tracing::warn!("Failed to create temp dir '{}': {}", path.display(), e);
        }
        let instance_id = format!(
            "pid{}_{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        );
        let lease_path = path.join(Self::lease_name(&instance_id));
        let lease = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lease_path)
            .expect("pADA temp sandbox lease must be creatable");
        Self::lock_exclusive(&lease).expect("pADA temp sandbox lease must be lockable");
        let sandbox = Self {
            temp_dir: path,
            instance_id,
            lease_path,
            lease,
        };
        sandbox.startup_gc();
        sandbox
    }
    pub fn db_path(&self) -> PathBuf {
        self.temp_dir.join(Self::db_name(&self.instance_id))
    }
    fn db_name(instance_id: &str) -> String {
        format!("session_{instance_id}_scratch.db")
    }
    fn lease_name(instance_id: &str) -> String {
        format!("{}.lease", Self::db_name(instance_id))
    }
    fn session_db_name(file_name: &str) -> Option<&str> {
        let (base, suffix) = file_name.split_once(".db")?;
        if !base.starts_with("session_") || !base.ends_with("_scratch") {
            return None;
        }
        if suffix.is_empty() || suffix.starts_with('.') {
            Some(&file_name[..base.len() + 3])
        } else {
            None
        }
    }
    fn lock_exclusive(file: &std::fs::File) -> std::io::Result<()> {
        fs2::FileExt::try_lock_exclusive(file)
    }
    fn unlock(file: &std::fs::File) {
        let _ = fs2::FileExt::unlock(file);
    }
    pub fn try_lock(path: &Path) -> Option<std::fs::File> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .ok()?;
        Self::lock_exclusive(&file).ok()?;
        Some(file)
    }
    fn startup_gc(&self) {
        tracing::debug!(
            "Running startup GC for temp directory '{}'",
            self.temp_dir.display()
        );
        Self::gc_orphans(&self.temp_dir);
    }
    pub fn gc_orphans(dir: &Path) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Cannot read dir for GC: {}", e);
                return;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(db_name) = Self::session_db_name(file_name) else {
                continue;
            };
            let lease_path = dir.join(format!("{db_name}.lease"));
            if lease_path.exists() && Self::try_lock(&lease_path).is_none() {
                tracing::debug!("Session '{}' has a live lease, skipping GC", db_name);
                continue;
            }
            match fs::remove_file(&path) {
                Ok(_) => tracing::info!("GC removed orphan temp file: {}", path.display()),
                Err(e) => tracing::warn!(
                    "GC: could not remove '{}' ({}). This is harmless.",
                    path.display(),
                    e
                ),
            }
        }
    }
    pub fn path(&self) -> &Path {
        &self.temp_dir
    }
    pub fn cleanup(&self) {
        Self::unlock(&self.lease);
        let db_name = Self::db_name(&self.instance_id);
        if let Ok(entries) = fs::read_dir(&self.temp_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let matches = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| {
                        name == self
                            .lease_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            || name == db_name
                            || name.starts_with(&format!("{db_name}."))
                    })
                    .unwrap_or(false);
                if !matches {
                    continue;
                }
                if let Err(e) = fs::remove_file(&path) {
                    tracing::warn!("Cleanup: could not remove '{}': {}", path.display(), e);
                }
            }
        }
        if let Ok(mut entries) = fs::read_dir(&self.temp_dir) {
            if entries.next().is_none() {
                let _ = fs::remove_dir(&self.temp_dir);
            }
        }
    }
}
