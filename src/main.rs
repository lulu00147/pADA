#![forbid(unsafe_code)]
use clap::Parser;
use pada::config::Config;
use pada::engine::TempSandbox;
use pada::engine::{EngineActor, TimedActor};
use pada::mcp::default_handlers;
use pada::mcp::Router;
use pada::transport;
use std::fs::create_dir_all;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
const GC_INTERVAL_SECS: u64 = 600;
const SHUTDOWN_MAX_WAIT_SECS: u64 = 15;
fn prepare_workspace_file(path: &std::path::Path, format: &str) -> Result<(), String> {
    if matches!(format, "csv" | "tsv") {
        pada::security::ensure_utf8_inplace(path)?;
    }
    Ok(())
}
#[derive(Parser)]
#[command(name = "pADA", about = "pADA server for AI assistants")]
struct Cli {
    #[arg(short = 'c', long = "config", default_value = "config.toml")]
    config: String,
}
fn init_logging(config: &Config) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.logging.level));
    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(filter.clone());
    let subscriber = tracing_subscriber::Registry::default().with(stderr_layer);
    if !config.logging.file_path.is_empty() {
        if let Some(parent) = std::path::Path::new(&config.logging.file_path).parent() {
            if let Err(e) = create_dir_all(parent) {
                tracing::warn!("Failed to create log directory: {}", e);
            }
        }
        match std::fs::File::create(&config.logging.file_path) {
            Ok(file) => {
                let (non_blocking, guard) = tracing_appender::non_blocking(file);
                let _ = Box::leak(Box::new(guard));
                let file_layer = tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_filter(filter);
                tracing::subscriber::set_global_default(subscriber.with(file_layer))
                    .unwrap_or_else(|e| {
                        tracing::warn!("Logging subscriber already set (idempotent call): {}", e)
                    });
            }
            Err(e) => {
                eprintln!(
                    "Warning: could not open log file '{}': {}",
                    config.logging.file_path, e
                );
                tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
                    tracing::warn!("Logging subscriber already set (idempotent call): {}", e)
                });
            }
        }
    } else {
        tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
            eprintln!("Warning: could not set logging default subscriber: {}", e);
        });
    }
}
const POLL_INTERVAL_MS: u64 = 100;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config_path = cli.config;
    let config = match Config::load(&config_path) {
        Ok(Some(cfg)) => cfg,
        Ok(None) => Config::default(),
        Err(e) => return Err(format!("Could not load {}: {}", config_path, e).into()),
    };
    init_logging(&config);
    std::fs::create_dir_all(&config.server.workspace_dir).map_err(|e| {
        format!(
            "Could not create workspace directory '{}': {}",
            config.server.workspace_dir, e
        )
    })?;
    tracing::info!("pADA server starting...");
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tracing::error!(target: "panic", "{}", panic_info);
        default_hook(panic_info);
    }));
    let engine = EngineActor::spawn(config.clone());
    let poll_start = std::time::Instant::now();
    let max_wait = Duration::from_secs(SHUTDOWN_MAX_WAIT_SECS);
    let mut engine_ready = false;
    loop {
        if engine.is_available() {
            engine_ready = true;
            break;
        }
        let elapsed = poll_start.elapsed();
        if elapsed >= max_wait {
            break;
        }
        let remaining = max_wait - elapsed;
        let sleep_for = std::cmp::min(Duration::from_millis(POLL_INTERVAL_MS), remaining);
        tokio::time::sleep(sleep_for).await;
    }
    if !engine_ready {
        tracing::warn!("DuckDB engine failed to start within {} s. Server will run in degraded mode -- all queries will fail.", max_wait.as_secs());
    }
    let timed_actor = Arc::new(TimedActor::new(
        engine,
        config.server.query_timeout_secs,
        config.server.spatial_query_timeout_secs,
        config.server.max_concurrent_queries,
        config.server.cache_capacity,
    ));
    let gc_temp_dir = config.server.temp_dir.clone();
    let gc_config = config.server.clone();
    let gc_actor = timed_actor.clone();
    let gc_interval = if config.server.gc_interval_secs == 0 {
        Duration::from_secs(GC_INTERVAL_SECS)
    } else {
        Duration::from_secs(config.server.gc_interval_secs)
    };
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(gc_interval);
        loop {
            interval.tick().await;
            tracing::debug!("Background GC scanning temp dir '{}'", gc_temp_dir);
            let dir = gc_temp_dir.clone();
            if let Err(e) = tokio::task::spawn_blocking(move || {
                TempSandbox::gc_orphans(std::path::Path::new(&dir));
            })
            .await
            {
                tracing::warn!("Background GC task failed: {}", e);
            }
            if !gc_actor.is_available() {
                continue;
            }
            let files =
                pada::utils::ttl::sweep_expired_files(&gc_actor, &gc_config, SystemTime::now())
                    .await;
            let (tables_dropped, table_warnings) =
                pada::mcp::tools::scratchpad::expire_scratchpad_tables(
                    &gc_actor,
                    &gc_config.state_dir,
                    Duration::from_secs(gc_config.temp_table_ttl_secs),
                )
                .await;
            tracing::debug!(
                "Background GC: TTL removed {} data file(s) and {} scratchpad table(s), {} failure(s)",
                files.removed.len(),
                tables_dropped,
                files.failures.len() + table_warnings.len()
            );
        }
    });
    let server_config = Arc::new(config.server.clone());
    let router = Router::new(server_config.clone(), timed_actor.clone());
    let tools = default_handlers();
    for tool in tools {
        router.register_tool(tool).await;
    }
    let router = Arc::new(router);
    let workspace_dir = server_config.workspace_dir.clone();
    let state_dir = server_config.state_dir.clone();
    let scan_actor = timed_actor.clone();
    if let Ok(entries) = std::fs::read_dir(&workspace_dir) {
        let mut count = 0u32;
        const MAX_AUTO_REGISTER_FILES: u32 = 500;
        for entry in entries.flatten() {
            if count >= MAX_AUTO_REGISTER_FILES {
                break;
            }
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') || name.starts_with("vw_") {
                continue;
            }
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            let fmt = match ext.as_str() {
                "csv" | "tsv" => "csv",
                "parquet" | "pq" => "parquet",
                "xlsx" => "excel",
                "geojson" => "geojson",
                "shp" => "shp",
                "gpkg" => "gpkg",
                "json" | "jsonl" | "ndjson" => "json",
                _ => continue,
            };
            if let Err(e) = prepare_workspace_file(&path, fmt) {
                tracing::warn!("Auto-register skipped for '{}': {}", path.display(), e);
                continue;
            }
            if let Err(e) = pada::utils::register_view_for_file(
                &path,
                std::path::Path::new(&state_dir),
                fmt,
                None,
                None,
                &scan_actor,
            )
            .await
            {
                tracing::warn!("Auto-register failed for '{}': {}", path.display(), e);
            } else {
                count += 1;
            }
        }
        if count > 0 {
            tracing::info!("Auto-registered {} data file(s) from workspace", count);
        }
    }
    let server_handle = {
        let r = router.clone();
        let cfg = Arc::new(config);
        let transport = cfg.server.transport.to_lowercase();
        tokio::spawn(async move {
            match transport.as_str() {
                "http" | "streamable-http" => {
                    tracing::info!(
                        "pADA server ready (Streamable HTTP). Awaiting HTTP connections..."
                    );
                    if let Err(e) = transport::run_http_server(cfg, r).await {
                        tracing::error!("HTTP server error: {}", e);
                    }
                }
                _ => {
                    tracing::info!("pADA server ready (stdio). Awaiting MCP stdio commands...");
                    if let Err(e) = pada::mcp::stdio::run(r).await {
                        tracing::error!("Server error: {}", e);
                    }
                }
            }
        })
    };
    tokio::select! {
        _ = server_handle => {
            tracing::info!("Server stopped.");
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received SIGINT (Ctrl+C), shutting down gracefully...");
        }
    }
    timed_actor.shutdown();
    tracing::info!("pADA server shutdown complete.");
    Ok(())
}
