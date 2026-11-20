use std::path::{Path, PathBuf};
fn canonical_workspace(workspace_dir: &str) -> Result<PathBuf, String> {
    let workspace_path = Path::new(workspace_dir);
    if workspace_path.is_absolute() {
        dunce::canonicalize(workspace_path)
            .map_err(|_| "Workspace directory is unavailable".to_string())
    } else {
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Cannot determine current directory: {}", e))?;
        dunce::canonicalize(cwd.join(workspace_path))
            .map_err(|_| "Workspace directory is unavailable".to_string())
    }
}
fn is_within_workspace(path: &Path, workspace: &Path) -> bool {
    path.starts_with(workspace)
}
pub fn validate_path(file_path: &str, workspace_dir: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    let canonical =
        dunce::canonicalize(path).map_err(|_| "Cannot resolve the requested path".to_string())?;
    let workspace_canonical = canonical_workspace(workspace_dir)?;
    if !is_within_workspace(&canonical, &workspace_canonical) {
        return Err("Access denied: the requested path is outside the workspace".into());
    }
    Ok(canonical.to_string_lossy().to_string())
}
pub fn resolve_registration_path(file_path: &str, workspace_dir: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    let canonical =
        dunce::canonicalize(path).map_err(|_| "Cannot resolve the requested path".to_string())?;
    let workspace_canonical = canonical_workspace(workspace_dir)?;
    if is_within_workspace(&canonical, &workspace_canonical) {
        return Ok(canonical.to_string_lossy().to_string());
    }
    Err("Access denied: the requested path is outside the workspace. Please copy the file into the workspace and re-register.".into())
}
