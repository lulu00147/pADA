use serde::{Deserialize, Serialize};
pub const MAX_TTL_SECS: u64 = 315_360_000;
pub fn exe_dir() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    exe.parent().map(|p| p.to_path_buf())
}
pub fn resolve_relative(path: &str, base: &std::path::Path) -> String {
    let p = std::path::Path::new(path);
    if p.is_relative() {
        base.join(p).to_string_lossy().to_string()
    } else {
        path.to_string()
    }
}
fn canonical_config_path(
    path: &std::path::Path,
) -> Result<std::path::PathBuf, crate::config::ConfigError> {
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| crate::config::ConfigError::Io(e.to_string()))?
            .join(path)
    };
    if let Ok(canonical) = dunce::canonicalize(&abs) {
        return Ok(canonical);
    }
    let parent = abs.parent().ok_or_else(|| {
        crate::config::ConfigError::Validation(format!("cannot resolve path '{}'", abs.display()))
    })?;
    let leaf = abs.file_name().ok_or_else(|| {
        crate::config::ConfigError::Validation(format!("invalid path '{}'", abs.display()))
    })?;
    let canonical_parent = dunce::canonicalize(parent).map_err(|e| {
        crate::config::ConfigError::Io(format!("cannot canonicalize '{}': {e}", parent.display()))
    })?;
    Ok(canonical_parent.join(leaf))
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub workspace_dir: String,
    pub temp_dir: String,
    #[serde(default)]
    pub state_dir: String,
    #[serde(default)]
    pub extension_directory: String,
    pub max_memory: String,
    pub stream_limit: usize,
    #[serde(default = "default_max_result_bytes")]
    pub max_result_bytes: usize,
    pub query_timeout_secs: u64,
    pub spatial_query_timeout_secs: u64,
    pub max_columns: usize,
    pub cell_truncate_chars: usize,
    pub excel_max_size_mb: u64,
    pub allowed_extensions: Vec<String>,
    #[serde(default = "default_transport")]
    pub transport: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: String,
    #[serde(default)]
    pub http_bearer_token: String,
    #[serde(default)]
    pub allow_remote_http: bool,
    #[serde(default = "default_max_concurrent_queries")]
    pub max_concurrent_queries: u32,
    #[serde(default = "default_cache_capacity")]
    pub cache_capacity: usize,
    #[serde(default = "default_gc_interval_secs")]
    pub gc_interval_secs: u64,
    #[serde(default = "default_temp_table_ttl_secs")]
    pub temp_table_ttl_secs: u64,
    #[serde(default = "default_data_file_ttl_secs")]
    pub data_file_ttl_secs: u64,
    #[serde(default = "default_underscore_data_file_ttl_secs")]
    pub underscore_data_file_ttl_secs: u64,
}
fn default_transport() -> String {
    "stdio".into()
}
fn default_host() -> String {
    "127.0.0.1".into()
}
fn default_port() -> u16 {
    3100
}
fn default_allowed_origins() -> String {
    "http://localhost:3100".into()
}
fn default_max_concurrent_queries() -> u32 {
    8
}
fn default_cache_capacity() -> usize {
    128
}
fn default_gc_interval_secs() -> u64 {
    600
}
fn default_max_result_bytes() -> usize {
    10 * 1024 * 1024
}
fn default_temp_table_ttl_secs() -> u64 {
    86_400
}
fn default_data_file_ttl_secs() -> u64 {
    604_800
}
fn default_underscore_data_file_ttl_secs() -> u64 {
    31_536_000
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            workspace_dir: "./data_workspace".into(),
            temp_dir: "./.duckdb_tmp".into(),
            state_dir: "./.duckdb_state".into(),
            extension_directory: "".into(),
            max_memory: "4GB".into(),
            stream_limit: 5000,
            max_result_bytes: default_max_result_bytes(),
            query_timeout_secs: 60,
            spatial_query_timeout_secs: 120,
            max_columns: 500,
            cell_truncate_chars: 1000,
            excel_max_size_mb: 500,
            allowed_extensions: vec!["spatial".into(), "excel".into()],
            transport: default_transport(),
            host: default_host(),
            port: default_port(),
            allowed_origins: default_allowed_origins(),
            http_bearer_token: String::new(),
            allow_remote_http: false,
            max_concurrent_queries: default_max_concurrent_queries(),
            cache_capacity: default_cache_capacity(),
            gc_interval_secs: default_gc_interval_secs(),
            temp_table_ttl_secs: default_temp_table_ttl_secs(),
            data_file_ttl_secs: default_data_file_ttl_secs(),
            underscore_data_file_ttl_secs: default_underscore_data_file_ttl_secs(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: String,
}
impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".into(),
            file_path: "./logs/pADA.log".into(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}
impl Config {
    pub fn from_file(path: &str) -> Result<Self, crate::config::ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::config::ConfigError::Io(e.to_string()))?;
        toml::from_str(&content).map_err(|e| crate::config::ConfigError::Parse(e.to_string()))
    }
    pub fn validate(&self) -> Result<(), crate::config::ConfigError> {
        let server = &self.server;
        match server.transport.to_ascii_lowercase().as_str() {
            "stdio" | "http" | "streamable-http" => {}
            other => {
                return Err(crate::config::ConfigError::Validation(format!(
                    "unsupported transport '{other}'; expected stdio, http, or streamable-http"
                )))
            }
        }
        for (name, value, maximum) in [
            ("query_timeout_secs", server.query_timeout_secs, 86_400),
            (
                "spatial_query_timeout_secs",
                server.spatial_query_timeout_secs,
                86_400,
            ),
            ("gc_interval_secs", server.gc_interval_secs, 86_400),
        ] {
            if value == 0 || value > maximum {
                return Err(crate::config::ConfigError::Validation(format!(
                    "{name} must be between 1 and {maximum}"
                )));
            }
        }
        if server.max_concurrent_queries == 0 || server.max_concurrent_queries > 128 {
            return Err(crate::config::ConfigError::Validation(
                "max_concurrent_queries must be between 1 and 128".into(),
            ));
        }
        for (name, value) in [
            ("temp_table_ttl_secs", server.temp_table_ttl_secs),
            ("data_file_ttl_secs", server.data_file_ttl_secs),
            (
                "underscore_data_file_ttl_secs",
                server.underscore_data_file_ttl_secs,
            ),
        ] {
            if value > MAX_TTL_SECS {
                return Err(crate::config::ConfigError::Validation(format!(
                    "{name} must be between 0 and {MAX_TTL_SECS} (0 disables expiry)"
                )));
            }
        }
        for (name, value, maximum) in [
            ("stream_limit", server.stream_limit, 100_000),
            (
                "max_result_bytes",
                server.max_result_bytes,
                100 * 1024 * 1024,
            ),
            ("max_columns", server.max_columns, 10_000),
            ("cell_truncate_chars", server.cell_truncate_chars, 1_000_000),
        ] {
            if value == 0 || value > maximum {
                return Err(crate::config::ConfigError::Validation(format!(
                    "{name} must be between 1 and {maximum}"
                )));
            }
        }
        if server.cache_capacity > 10_000 {
            return Err(crate::config::ConfigError::Validation(
                "cache_capacity must not exceed 10000".into(),
            ));
        }
        if server.excel_max_size_mb == 0 || server.excel_max_size_mb > 10_240 {
            return Err(crate::config::ConfigError::Validation(
                "excel_max_size_mb must be between 1 and 10240".into(),
            ));
        }
        if server.max_memory.trim().is_empty() {
            return Err(crate::config::ConfigError::Validation(
                "max_memory must not be empty".into(),
            ));
        }
        let workspace = canonical_config_path(std::path::Path::new(&server.workspace_dir))?;
        let temp = canonical_config_path(std::path::Path::new(&server.temp_dir))?;
        let state = canonical_config_path(std::path::Path::new(&server.state_dir))?;
        for (left_name, left, right_name, right) in [
            ("workspace_dir", &workspace, "temp_dir", &temp),
            ("workspace_dir", &workspace, "state_dir", &state),
            ("temp_dir", &temp, "state_dir", &state),
        ] {
            if left.starts_with(right) || right.starts_with(left) {
                return Err(crate::config::ConfigError::Validation(format!(
                    "{left_name} ('{}') must not overlap {right_name} ('{}')",
                    left.display(),
                    right.display()
                )));
            }
        }
        Ok(())
    }
    pub fn load(filename: &str) -> Result<Option<Self>, crate::config::ConfigError> {
        let paths = [
            std::path::PathBuf::from(filename),
            exe_dir().map(|d| d.join(filename)).unwrap_or_default(),
        ];
        for path in &paths {
            if path.exists() {
                let config_dir = path.parent().unwrap_or(std::path::Path::new("."));
                let mut config = Config::from_file(&path.to_string_lossy())?;
                config.server.workspace_dir =
                    resolve_relative(&config.server.workspace_dir, config_dir);
                config.server.temp_dir = resolve_relative(&config.server.temp_dir, config_dir);
                if config.server.state_dir.is_empty() {
                    config.server.state_dir = resolve_relative("./.duckdb_state", config_dir);
                } else {
                    config.server.state_dir =
                        resolve_relative(&config.server.state_dir, config_dir);
                }
                if !config.server.extension_directory.is_empty() {
                    config.server.extension_directory =
                        resolve_relative(&config.server.extension_directory, config_dir);
                }
                if !config.logging.file_path.is_empty() {
                    config.logging.file_path =
                        resolve_relative(&config.logging.file_path, config_dir);
                }
                config.validate()?;
                return Ok(Some(config));
            }
        }
        Ok(None)
    }
}
#[derive(Debug)]
pub enum ConfigError {
    Io(String),
    Parse(String),
    Validation(String),
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "IO error: {}", msg),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}
impl std::error::Error for ConfigError {}
