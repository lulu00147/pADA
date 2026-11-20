use crate::security::extension_guard::BLOCKED_EXTENSIONS;
use std::path::Path;
pub struct SpatialExtension;
impl SpatialExtension {
    pub fn init_sql(allowed: &[String], extension_dir: Option<&str>) -> Vec<String> {
        let mut statements = Vec::new();
        let ext_dir = extension_dir
            .map(|d| d.to_string())
            .unwrap_or_else(Self::default_extension_dir);
        for ext in allowed {
            let lower = ext.to_lowercase();
            if BLOCKED_EXTENSIONS.contains(&lower.as_str()) {
                continue;
            }
            let ext_path = Path::new(&ext_dir).join(format!("{}.duckdb_extension", lower));
            let safe_path = ext_path.display().to_string().replace('\'', "''");
            statements.push(format!("LOAD '{}';", safe_path));
        }
        statements
    }
    pub fn validate_files(
        allowed: &[String],
        extension_dir: Option<&str>,
    ) -> Result<(), Vec<String>> {
        let ext_dir = extension_dir
            .map(|d| d.to_string())
            .unwrap_or_else(Self::default_extension_dir);
        let mut missing = Vec::new();
        for ext in allowed {
            let lower = ext.to_lowercase();
            if BLOCKED_EXTENSIONS.contains(&lower.as_str()) {
                continue;
            }
            let ext_path = Path::new(&ext_dir).join(format!("{}.duckdb_extension", lower));
            if !ext_path.exists() {
                missing.push(ext.clone());
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    pub fn default_extension_dir() -> String {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                return parent.to_string_lossy().to_string();
            }
        }
        ".".into()
    }
}
