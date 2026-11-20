pub struct ExtensionGuard;
pub(crate) const BLOCKED_EXTENSIONS: &[&str] =
    &["httpfs", "azure", "aws", "s3", "s3fs", "delta", "iceberg"];
pub(crate) const APPROVED_EXTENSIONS: &[&str] = &["spatial", "excel"];
impl ExtensionGuard {
    pub fn init_sql() -> Vec<String> {
        vec![
            "SET allow_unsigned_extensions=false;".into(),
            "SET autoinstall_known_extensions=false;".into(),
            "SET autoload_known_extensions=false;".into(),
        ]
    }
    pub fn validate_allowed(allowed: &[String]) -> Result<(), String> {
        for ext in allowed {
            let lower = ext.to_lowercase();
            if BLOCKED_EXTENSIONS.contains(&lower.as_str()) {
                return Err(format!(
                    "Extension '{}' is blocked for security reasons (SSRF prevention).",
                    ext
                ));
            }
            if !APPROVED_EXTENSIONS.contains(&lower.as_str()) {
                return Err(format!(
                    "Extension '{}' is not in the server sidecar extension allowlist.",
                    ext
                ));
            }
        }
        Ok(())
    }
}
