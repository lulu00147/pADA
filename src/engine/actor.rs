use crate::config::Config;
#[cfg(feature = "duckdb-impl")]
use crate::engine::duckdb_value_to_json;
use crate::engine::temp_sandbox::TempSandbox;
use crate::engine::types::{OperationState, StructuredResultStream, RESULT_BATCH_ROWS};
use crate::engine::EngineError;
#[cfg(feature = "duckdb-impl")]
use crate::security::ExtensionGuard;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
pub enum ActorCommand {
    Read {
        sql: String,
        operation_id: Option<String>,
        respond: tokio::sync::oneshot::Sender<Result<String, EngineError>>,
    },
    ValidateNative {
        sql: String,
        operation_id: Option<String>,
        use_write_conn: bool,
        respond: tokio::sync::oneshot::Sender<Result<(), EngineError>>,
    },
    #[allow(clippy::type_complexity)]
    ReadStructured {
        sql: String,
        operation_id: Option<String>,
        use_write_conn: bool,
        respond: tokio::sync::oneshot::Sender<Result<StructuredResultStream, EngineError>>,
    },
    Write {
        sql: String,
        operation_id: Option<String>,
        respond: tokio::sync::oneshot::Sender<Result<String, EngineError>>,
    },
    RegisterView {
        view_name: String,
        sql: String,
        operation_id: Option<String>,
        respond: tokio::sync::oneshot::Sender<Result<(), EngineError>>,
    },
    Shutdown,
}
const ACTOR_CHANNEL_CAP: usize = 64;
const DEFAULT_THREADS: u32 = 4;
const FINISHED_OPERATION_CAPACITY: usize = 1024;
#[derive(Default)]
pub(crate) struct OperationRegistry {
    states: HashMap<String, OperationState>,
    finished: VecDeque<String>,
}
impl OperationRegistry {
    fn queue(&mut self, id: String) {
        self.finished.retain(|finished| finished != &id);
        self.states.insert(id, OperationState::Queued);
    }
    fn state(&self, id: &str) -> Option<OperationState> {
        self.states.get(id).copied()
    }
    fn request_interrupt(&mut self, id: &str) -> Option<bool> {
        match self.states.get_mut(id) {
            Some(state @ OperationState::Running) => {
                *state = OperationState::InterruptRequested;
                Some(true)
            }
            Some(state @ OperationState::Queued) => {
                *state = OperationState::InterruptRequested;
                Some(false)
            }
            Some(OperationState::InterruptRequested) => Some(false),
            Some(OperationState::Finished) | None => None,
        }
    }
    fn begin(&mut self, id: &str) -> bool {
        match self.state(id) {
            Some(OperationState::InterruptRequested) => {
                self.finish(id.to_string());
                false
            }
            Some(OperationState::Queued)
            | Some(OperationState::Running)
            | Some(OperationState::Finished)
            | None => {
                self.states.insert(id.to_string(), OperationState::Running);
                true
            }
        }
    }
    fn finish(&mut self, id: String) {
        self.states.insert(id.clone(), OperationState::Finished);
        self.finished.push_back(id);
        while self.finished.len() > FINISHED_OPERATION_CAPACITY {
            if let Some(expired) = self.finished.pop_front() {
                if self.states.get(&expired) == Some(&OperationState::Finished) {
                    self.states.remove(&expired);
                }
            }
        }
    }
}
const MAX_ENGINE_RESULT_ROWS: usize = 50_000;
#[cfg(feature = "duckdb-impl")]
fn ensure_result_row_limit(row_count: usize) -> Result<(), EngineError> {
    if row_count >= MAX_ENGINE_RESULT_ROWS {
        return Err(EngineError::internal(format!(
            "Query result exceeds the engine safety limit of {} rows",
            MAX_ENGINE_RESULT_ROWS
        )));
    }
    Ok(())
}
struct ActorAvailabilityGuard(Arc<AtomicBool>);
impl Drop for ActorAvailabilityGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}
pub struct EngineActor {
    tx: mpsc::Sender<ActorCommand>,
    operations: Arc<Mutex<OperationRegistry>>,
    fire_interrupt: Arc<OnceLock<Box<dyn Fn() + Send + Sync>>>,
    available: Arc<AtomicBool>,
    shutdown_requested: Arc<AtomicBool>,
    temp_tables: Arc<Mutex<HashMap<String, Instant>>>,
}
struct OperationRegistration {
    operations: Arc<Mutex<OperationRegistry>>,
    id: Option<String>,
}
impl OperationRegistration {
    fn new(operations: Arc<Mutex<OperationRegistry>>, id: Option<String>) -> Self {
        if let Some(operation_id) = &id {
            operations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .queue(operation_id.clone());
        }
        Self { operations, id }
    }
    fn disarm(mut self) {
        self.id = None;
    }
}
impl Drop for OperationRegistration {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            self.operations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .finish(id);
        }
    }
}
impl EngineActor {
    pub fn spawn(config: Config) -> Self {
        let (tx, mut rx) = mpsc::channel::<ActorCommand>(ACTOR_CHANNEL_CAP);
        let active_operation = Arc::new(Mutex::new(None::<String>));
        let active_for_thread = active_operation.clone();
        let operations = Arc::new(Mutex::new(OperationRegistry::default()));
        let operations_for_thread = operations.clone();
        let fire_interrupt = Arc::new(OnceLock::new());
        let fire_for_thread = fire_interrupt.clone();
        let available = Arc::new(AtomicBool::new(false));
        let avail_flag = available.clone();
        let shutdown_requested = Arc::new(AtomicBool::new(false));
        let shutdown_for_thread = shutdown_requested.clone();
        std::thread::spawn(move || {
            let _availability_guard = ActorAvailabilityGuard(avail_flag.clone());
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let temp_dir = std::path::Path::new(&config.server.temp_dir);
                let sandbox = TempSandbox::new(temp_dir, &config.server.workspace_dir);
                #[cfg(feature = "duckdb-impl")]
                let stub_only = match try_connect_duckdb(&config, &sandbox) {
                    Ok((read_conn, write_conn)) => {
                        let _ = fire_for_thread.set(make_interrupt_fn(&write_conn, &read_conn));
                        publish_availability(&avail_flag, &shutdown_for_thread);
                        tracing::info!(
                            "Engine actor started with DuckDB (temp: {})",
                            sandbox.db_path().display()
                        );
                        run_actor_loop(
                            &mut rx,
                            read_conn,
                            write_conn,
                            &active_for_thread,
                            &operations_for_thread,
                            &sandbox,
                        );
                        false
                    }
                    Err(e) => {
                        avail_flag.store(false, Ordering::SeqCst);
                        tracing::warn!(
                            "Engine actor started WITHOUT DuckDB ({}). All queries will fail.",
                            e
                        );
                        true
                    }
                };
                #[cfg(not(feature = "duckdb-impl"))]
                let stub_only: bool = {
                    let why = try_connect_duckdb(&config, &sandbox).unwrap_err();
                    tracing::warn!(
                        "Engine actor built WITHOUT DuckDB ({}). All queries will fail.",
                        why
                    );
                    true
                };
                if stub_only {
                    run_stub_loop(&mut rx, &operations_for_thread, &sandbox);
                }
                sandbox.cleanup();
                tracing::info!("Engine actor stopped.");
            }));
            if let Err(panic) = result {
                tracing::error!("duckdb-engine thread panicked: {:?}", panic);
            }
        });
        Self {
            tx,
            operations,
            fire_interrupt,
            available,
            shutdown_requested,
            temp_tables: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }
    fn mark_queued(&self, operation_id: &Option<String>) {
        if let Some(id) = operation_id {
            self.operations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .queue(id.clone());
        }
    }
    fn mark_finished(&self, operation_id: &Option<String>) {
        if let Some(id) = operation_id {
            self.operations
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .finish(id.clone());
        }
    }
    pub async fn read(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<String, EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        let registration =
            OperationRegistration::new(self.operations.clone(), operation_id.clone());
        self.tx
            .send(ActorCommand::Read {
                sql,
                operation_id,
                respond,
            })
            .await
            .map_err(|_| EngineError::shutdown("Actor channel closed"))?;
        registration.disarm();
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn validate_native(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<(), EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        let registration =
            OperationRegistration::new(self.operations.clone(), operation_id.clone());
        self.tx
            .send(ActorCommand::ValidateNative {
                sql,
                operation_id,
                use_write_conn: false,
                respond,
            })
            .await
            .map_err(|_| EngineError::shutdown("Actor channel closed"))?;
        registration.disarm();
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn validate_native_on_write(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<(), EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        let registration =
            OperationRegistration::new(self.operations.clone(), operation_id.clone());
        self.tx
            .send(ActorCommand::ValidateNative {
                sql,
                operation_id,
                use_write_conn: true,
                respond,
            })
            .await
            .map_err(|_| EngineError::shutdown("Actor channel closed"))?;
        registration.disarm();
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn read_structured(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<StructuredResultStream, EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        self.mark_queued(&operation_id);
        let queued_operation_id = operation_id.clone();
        self.tx
            .send(ActorCommand::ReadStructured {
                sql,
                operation_id,
                use_write_conn: false,
                respond,
            })
            .await
            .map_err(|_| {
                self.mark_finished(&queued_operation_id);
                EngineError::shutdown("Actor channel closed")
            })?;
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn read_structured_on_write(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<StructuredResultStream, EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        self.mark_queued(&operation_id);
        let queued_operation_id = operation_id.clone();
        self.tx
            .send(ActorCommand::ReadStructured {
                sql,
                operation_id,
                use_write_conn: true,
                respond,
            })
            .await
            .map_err(|_| {
                self.mark_finished(&queued_operation_id);
                EngineError::shutdown("Actor channel closed")
            })?;
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn write(
        &self,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<String, EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        self.mark_queued(&operation_id);
        let queued_operation_id = operation_id.clone();
        self.tx
            .send(ActorCommand::Write {
                sql,
                operation_id,
                respond,
            })
            .await
            .map_err(|_| {
                self.mark_finished(&queued_operation_id);
                EngineError::shutdown("Actor channel closed")
            })?;
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub async fn register_view(
        &self,
        view_name: String,
        sql: String,
        operation_id: Option<String>,
    ) -> Result<(), EngineError> {
        let (respond, rx) = tokio::sync::oneshot::channel();
        self.mark_queued(&operation_id);
        let queued_operation_id = operation_id.clone();
        self.tx
            .send(ActorCommand::RegisterView {
                view_name,
                sql,
                operation_id,
                respond,
            })
            .await
            .map_err(|_| {
                self.mark_finished(&queued_operation_id);
                EngineError::shutdown("Actor channel closed")
            })?;
        rx.await
            .map_err(|_| EngineError::shutdown("Actor response dropped"))?
    }
    pub fn interrupt(&self, operation_id: &str) -> bool {
        let interrupt_running = self
            .operations
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .request_interrupt(operation_id);
        if interrupt_running == Some(true) {
            if let Some(f) = self.fire_interrupt.get() {
                f();
            }
        }
        interrupt_running.is_some()
    }
    pub fn operation_state(&self, operation_id: &str) -> Option<OperationState> {
        self.operations
            .lock()
            .ok()
            .and_then(|operations| operations.state(operation_id))
    }
    pub fn note_temp_table_created(&self, name: &str) {
        self.temp_tables
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(name.to_string(), Instant::now());
    }
    pub fn note_temp_table_dropped(&self, name: &str) {
        self.temp_tables
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(name);
    }
    pub fn expired_temp_tables(&self, ttl: Duration) -> Vec<(String, Instant)> {
        if ttl.is_zero() {
            return Vec::new();
        }
        let tables = self.temp_tables.lock().unwrap_or_else(|e| e.into_inner());
        let mut expired: Vec<(&String, &Instant)> = tables
            .iter()
            .filter(|(_, created)| created.elapsed() >= ttl)
            .collect();
        expired.sort_by_key(|(_, created)| **created);
        expired
            .into_iter()
            .map(|(name, created)| (name.clone(), *created))
            .collect()
    }
    pub fn temp_table_is_current(&self, name: &str, created: Instant) -> bool {
        self.temp_tables
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(name)
            .is_some_and(|current| *current == created)
    }
    pub fn shutdown(&self) {
        if self.shutdown_requested.swap(true, Ordering::SeqCst) {
            return;
        }
        self.available.store(false, Ordering::SeqCst);
        request_shutdown(&self.tx);
    }
}
fn publish_availability(available: &AtomicBool, shutdown_requested: &AtomicBool) {
    available.store(!shutdown_requested.load(Ordering::SeqCst), Ordering::SeqCst);
}
fn request_shutdown(tx: &mpsc::Sender<ActorCommand>) {
    match tx.try_send(ActorCommand::Shutdown) {
        Ok(()) => {}
        Err(mpsc::error::TrySendError::Full(shutdown)) => {
            tracing::debug!("Engine actor channel full; shutdown will be delivered after drain.");
            let tx = tx.clone();
            std::thread::spawn(move || {
                let _ = tx.blocking_send(shutdown);
            });
        }
        Err(mpsc::error::TrySendError::Closed(_)) => {
            tracing::debug!("Engine actor was already stopped.");
        }
    }
}
impl Drop for EngineActor {
    fn drop(&mut self) {
        self.shutdown();
    }
}
#[cfg(feature = "duckdb-impl")]
fn make_interrupt_fn(
    write_conn: &duckdb_crate::Connection,
    read_conn: &duckdb_crate::Connection,
) -> Box<dyn Fn() + Send + Sync> {
    let write_handle = write_conn.interrupt_handle();
    let read_handle = read_conn.interrupt_handle();
    Box::new(move || {
        write_handle.interrupt();
        read_handle.interrupt();
    })
}
#[cfg(feature = "duckdb-impl")]
fn try_connect_duckdb(
    config: &Config,
    sandbox: &TempSandbox,
) -> Result<(duckdb_crate::Connection, duckdb_crate::Connection), EngineError> {
    let db_path = sandbox.db_path();
    let open_config = duckdb_crate::Config::default()
        .access_mode(duckdb_crate::AccessMode::ReadWrite)
        .map_err(|e| EngineError::duckdb(format!("DuckDB config (access_mode): {}", e)))?;
    let db = duckdb_crate::Connection::open_with_flags(&db_path, open_config)
        .map_err(|e| EngineError::duckdb(format!("Failed to open DuckDB: {}", e)))?;
    let read_conn = db
        .try_clone()
        .map_err(|e| EngineError::duckdb(format!("Failed to clone read connection: {}", e)))?;
    let max_memory = &config.server.max_memory;
    let memory_limit = if max_memory.is_empty() {
        tracing::warn!("max_memory is empty, falling back to default '2GB'");
        "2GB".to_string()
    } else if !max_memory
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.')
    {
        tracing::warn!(
            "max_memory '{}' contains invalid characters, falling back to default '2GB'",
            max_memory
        );
        "2GB".to_string()
    } else {
        max_memory.clone()
    };
    let config_sql = format!(
        "SET memory_limit='{}';\nPRAGMA threads={DEFAULT_THREADS};",
        memory_limit
    );
    if let Err(e) = db.execute_batch(&config_sql) {
        tracing::warn!("Could not set DuckDB config: {}", e);
    }
    if let Err(e) = db.execute_batch("CREATE SCHEMA IF NOT EXISTS ai_scratchpad;") {
        tracing::warn!("Could not create ai_scratchpad schema: {}", e);
    }
    if let Err(e) = db.execute_batch("SET schema = 'ai_scratchpad';") {
        tracing::warn!("Could not set write schema: {}", e);
    }
    if let Err(e) = read_conn.execute_batch("SET schema = 'ai_scratchpad';") {
        tracing::warn!("Could not set read schema: {}", e);
    }
    let allowed_exts: Vec<String> = config
        .server
        .allowed_extensions
        .iter()
        .map(|s| s.to_string())
        .collect();
    ExtensionGuard::validate_allowed(&allowed_exts).map_err(EngineError::internal)?;
    for sql in ExtensionGuard::init_sql() {
        if let Err(e) = db.execute_batch(&sql) {
            tracing::warn!("Could not execute security SQL: {}", e);
        }
    }
    let ext_dir = if config.server.extension_directory.is_empty() {
        None
    } else {
        Some(&config.server.extension_directory as &str)
    };
    if let Err(missing) = crate::engine::SpatialExtension::validate_files(&allowed_exts, ext_dir) {
        tracing::warn!("Extension files not found for: {}.", missing.join(", "));
    }
    for sql in crate::engine::SpatialExtension::init_sql(&allowed_exts, ext_dir) {
        if let Err(e) = db.execute_batch(&sql) {
            tracing::warn!("Could not execute spatial extension SQL: {}", e);
        }
    }
    let state_dir = std::path::Path::new(&config.server.state_dir);
    std::fs::create_dir_all(state_dir).map_err(|e| {
        EngineError::internal(format!(
            "Failed to create state directory '{}': {e}",
            state_dir.display()
        ))
    })?;
    restore_persisted_views(state_dir, &db, &read_conn).map_err(EngineError::internal)?;
    Ok((read_conn, db))
}
#[cfg(feature = "duckdb-impl")]
fn restore_persisted_views(
    state_dir: &std::path::Path,
    write_conn: &duckdb_crate::Connection,
    read_conn: &duckdb_crate::Connection,
) -> Result<(), String> {
    let run_one = |conn: &duckdb_crate::Connection, sql: &str| -> Result<(), String> {
        let result = conn.execute_batch(sql).map_err(|e| e.to_string());
        if result.is_err() {
            match conn.execute_batch("ROLLBACK") {
                Ok(_) => {
                    tracing::debug!("ROLLBACK after restore error cleared aborted transaction")
                }
                Err(e) => tracing::debug!("ROLLBACK (no-op): {}", e),
            }
        }
        result
    };
    let views = crate::utils::load_persisted_views(state_dir)?;
    for entry in &views {
        if !restorable_entry_file_exists(entry) {
            tracing::warn!(
                "Skipping persisted view for missing data file: {}",
                entry
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("<unknown>")
            );
            continue;
        }
        for key in ["sql", "alias_sql"] {
            if let Some(sql) = entry.get(key).and_then(|v| v.as_str()) {
                run_one(write_conn, sql).map_err(|e| {
                    format!("Failed to restore {key} on write connection: {e}; statement: {sql}")
                })?;
                run_one(read_conn, sql).map_err(|e| {
                    format!("Failed to restore {key} on read connection: {e}; statement: {sql}")
                })?;
            }
        }
    }
    Ok(())
}
#[cfg(any(feature = "duckdb-impl", test))]
fn restorable_entry_file_exists(entry: &serde_json::Value) -> bool {
    match entry.get("file_path").and_then(|value| value.as_str()) {
        Some(path) => std::path::Path::new(path).exists(),
        None => true,
    }
}
#[cfg(not(feature = "duckdb-impl"))]
fn try_connect_duckdb(
    _config: &Config,
    _sandbox: &TempSandbox,
) -> Result<(String, String), String> {
    Err(
        "DuckDB feature not enabled. Rebuild with `--features duckdb` or provide libduckdb.a."
            .to_string(),
    )
}
#[cfg(feature = "duckdb-impl")]
fn run_actor_loop(
    rx: &mut mpsc::Receiver<ActorCommand>,
    read_conn: duckdb_crate::Connection,
    write_conn: duckdb_crate::Connection,
    active_operation: &Mutex<Option<String>>,
    operations: &Mutex<OperationRegistry>,
    _sandbox: &TempSandbox,
) {
    while let Some(cmd) = rx.blocking_recv() {
        match cmd {
            ActorCommand::Read {
                sql,
                operation_id,
                respond,
            } => {
                let _ = respond.send(execute_with_active(
                    active_operation,
                    operations,
                    operation_id,
                    || execute_read(&read_conn, &sql),
                ));
            }
            ActorCommand::ValidateNative {
                sql,
                operation_id,
                use_write_conn,
                respond,
            } => {
                let conn = if use_write_conn {
                    &write_conn
                } else {
                    &read_conn
                };
                let _ = respond.send(execute_with_active(
                    active_operation,
                    operations,
                    operation_id,
                    || validate_native(conn, &sql),
                ));
            }
            ActorCommand::ReadStructured {
                sql,
                operation_id,
                use_write_conn,
                respond,
            } => {
                let conn = if use_write_conn {
                    &write_conn
                } else {
                    &read_conn
                };
                execute_read_structured(
                    conn,
                    &sql,
                    active_operation,
                    operations,
                    operation_id,
                    respond,
                );
            }
            ActorCommand::Write {
                sql,
                operation_id,
                respond,
            } => {
                let _ = respond.send(execute_with_active(
                    active_operation,
                    operations,
                    operation_id,
                    || execute_write(&write_conn, &sql),
                ));
            }
            ActorCommand::RegisterView {
                view_name,
                sql,
                operation_id,
                respond,
            } => {
                let result = execute_with_active(
                    active_operation,
                    operations,
                    operation_id,
                    || -> Result<(), EngineError> {
                        execute_write(&write_conn, &sql)?;
                        if let Err(error) = read_conn.execute_batch(&sql) {
                            let rollback = format!(
                                "DROP VIEW IF EXISTS {}",
                                crate::utils::escape_ident(&view_name)
                            );
                            if let Err(rollback_error) = execute_write(&write_conn, &rollback) {
                                tracing::error!(
                                    "Failed to roll back write connection after read sync failure: {}",
                                    rollback_error
                                );
                            }
                            if let Err(rollback_error) = read_conn.execute_batch(&rollback) {
                                tracing::error!(
                                    "Failed to roll back read connection after sync failure: {}",
                                    rollback_error
                                );
                            }
                            tracing::error!("Read connection sync failed: {}", error);
                            return Err(EngineError::duckdb(
                                "View registration synchronization failed",
                            ));
                        }
                        Ok(())
                    },
                );
                if result.is_ok() {
                    tracing::info!("Registered view '{}'", view_name);
                }
                let _ = respond.send(result);
            }
            ActorCommand::Shutdown => {
                tracing::info!("Engine actor shutting down (DuckDB).");
                break;
            }
        }
    }
}
#[cfg(feature = "duckdb-impl")]
fn execute_with_active<T>(
    active_operation: &Mutex<Option<String>>,
    operations: &Mutex<OperationRegistry>,
    operation_id: Option<String>,
    execute: impl FnOnce() -> Result<T, EngineError>,
) -> Result<T, EngineError> {
    let active_id = operation_id.unwrap_or_else(internal_operation_id);
    {
        let mut states = operations.lock().unwrap_or_else(|e| e.into_inner());
        if !states.begin(&active_id) {
            return Err(EngineError::shutdown("Query cancelled before execution"));
        }
    }
    *active_operation.lock().unwrap_or_else(|e| e.into_inner()) = Some(active_id.clone());
    let result = execute();
    *active_operation.lock().unwrap_or_else(|e| e.into_inner()) = None;
    operations
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .finish(active_id);
    result
}
#[cfg(feature = "duckdb-impl")]
fn internal_operation_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("internal-{}-{}", std::process::id(), nanos)
}
fn finish_stub_operation(operations: &Mutex<OperationRegistry>, operation_id: Option<String>) {
    if let Some(id) = operation_id {
        operations
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .finish(id);
    }
}
pub(crate) fn run_stub_loop(
    rx: &mut mpsc::Receiver<ActorCommand>,
    operations: &Mutex<OperationRegistry>,
    _sandbox: &TempSandbox,
) {
    while let Some(cmd) = rx.blocking_recv() {
        match cmd {
            ActorCommand::Read {
                operation_id,
                respond,
                ..
            }
            | ActorCommand::Write {
                operation_id,
                respond,
                ..
            } => {
                finish_stub_operation(operations, operation_id);
                let _ = respond.send(Err(EngineError::shutdown("DuckDB engine is not available. Rebuild with the `duckdb` feature and provide libduckdb.a.")));
            }
            ActorCommand::ValidateNative {
                operation_id,
                respond,
                ..
            } => {
                finish_stub_operation(operations, operation_id);
                let _ = respond.send(Err(EngineError::shutdown("DuckDB engine is not available. Rebuild with the `duckdb` feature and provide libduckdb.a.")));
            }
            ActorCommand::ReadStructured {
                operation_id,
                respond,
                ..
            } => {
                finish_stub_operation(operations, operation_id);
                let _ = respond.send(Err(EngineError::shutdown("DuckDB engine is not available. Rebuild with the `duckdb` feature and provide libduckdb.a.")));
            }
            ActorCommand::RegisterView {
                operation_id,
                respond,
                ..
            } => {
                finish_stub_operation(operations, operation_id);
                let _ = respond.send(Err(EngineError::shutdown(
                    "DuckDB engine is not available. Cannot register views.",
                )));
            }
            ActorCommand::Shutdown => {
                tracing::info!("Engine actor shutting down (stub).");
                break;
            }
        }
    }
}
#[cfg(feature = "duckdb-impl")]
fn col_names(rows: &duckdb_crate::Rows) -> Vec<String> {
    rows.as_ref().map(|s| s.column_names()).unwrap_or_default()
}
#[cfg(feature = "duckdb-impl")]
fn with_rollback<T>(
    conn: &duckdb_crate::Connection,
    op: impl FnOnce() -> Result<T, EngineError>,
) -> Result<T, EngineError> {
    let result = op();
    if result.is_err() {
        match conn.execute_batch("ROLLBACK") {
            Ok(_) => tracing::debug!("ROLLBACK after error cleared aborted transaction"),
            Err(e) => tracing::debug!("ROLLBACK (no-op): {}", e),
        }
    }
    result
}
#[cfg(feature = "duckdb-impl")]
fn validate_native(conn: &duckdb_crate::Connection, sql: &str) -> Result<(), EngineError> {
    with_rollback(conn, || {
        conn.prepare(sql).map(drop).map_err(|e| {
            tracing::debug!("DuckDB native prepare validation failed");
            EngineError::duckdb(format!("DuckDB cannot prepare this SQL: {}", e))
        })
    })
}
#[cfg(feature = "duckdb-impl")]
fn execute_read(conn: &duckdb_crate::Connection, sql: &str) -> Result<String, EngineError> {
    with_rollback(conn, || {
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| EngineError::duckdb(format!("Prepare error: {}", e)))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| EngineError::duckdb(format!("Query error: {}", e)))?;
        let col_names = col_names(&rows);
        let mut results: Vec<serde_json::Value> = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|e| EngineError::duckdb(format!("Row error: {}", e)))?
        {
            ensure_result_row_limit(results.len())?;
            let mut obj = serde_json::Map::with_capacity(col_names.len());
            for (i, name) in col_names.iter().enumerate() {
                obj.insert(name.clone(), duckdb_value_to_json(row, i));
            }
            results.push(serde_json::Value::Object(obj));
        }
        serde_json::to_string(&results)
            .map_err(|e| EngineError::internal(format!("JSON serialization error: {}", e)))
    })
}
#[cfg(feature = "duckdb-impl")]
#[cfg(feature = "duckdb-impl")]
fn execute_write(conn: &duckdb_crate::Connection, sql: &str) -> Result<String, EngineError> {
    with_rollback(conn, || {
        let is_drop = sql
            .trim_start()
            .get(..5)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("DROP "));
        let tx_sql = if is_drop {
            sql.to_string()
        } else {
            format!("BEGIN;\n{}\nCOMMIT;", sql)
        };
        if let Err(e) = conn.execute_batch("ROLLBACK;") {
            tracing::trace!("execute_write: pre-write ROLLBACK no-op: {}", e);
        }
        tracing::trace!(
            "execute_write: {} chars, head={}",
            sql.chars().count(),
            sql.chars().take(200).collect::<String>().replace('\n', " ")
        );
        match conn.execute_batch(&tx_sql) {
            Ok(()) => Ok("{\"ok\":true}".to_string()),
            Err(e) => {
                let msg = e.to_string();
                tracing::debug!("execute_write failed: {}", msg);
                tracing::debug!("execute_write batch (verbatim) = {:?}", tx_sql);
                Err(if msg.len() > 500 {
                    let trunc_end = msg
                        .char_indices()
                        .map(|(i, _)| i)
                        .nth(500)
                        .unwrap_or(msg.len());
                    EngineError::duckdb(format!("Batch error: {}...", &msg[..trunc_end]))
                } else {
                    EngineError::duckdb(format!("Batch error: {}", msg))
                })
            }
        }
    })
}
#[cfg(feature = "duckdb-impl")]
fn execute_read_structured(
    conn: &duckdb_crate::Connection,
    sql: &str,
    active_operation: &Mutex<Option<String>>,
    operations: &Mutex<OperationRegistry>,
    operation_id: Option<String>,
    respond: tokio::sync::oneshot::Sender<Result<StructuredResultStream, EngineError>>,
) {
    let active_id = operation_id.unwrap_or_else(internal_operation_id);
    let mut respond = Some(respond);
    let mut stream_sender = None;
    {
        let mut states = operations.lock().unwrap_or_else(|e| e.into_inner());
        if !states.begin(&active_id) {
            let _ = respond
                .take()
                .expect("structured response is sent once")
                .send(Err(EngineError::shutdown(
                    "Query cancelled before execution",
                )));
            return;
        }
    }
    *active_operation.lock().unwrap_or_else(|e| e.into_inner()) = Some(active_id.clone());
    let result = with_rollback(conn, || {
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| EngineError::duckdb(format!("Prepare error: {}", e)))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| EngineError::duckdb(format!("Query error: {}", e)))?;
        let columns = col_names(&rows);
        let col_count = columns.len();
        let (tx, batches) = mpsc::channel(1);
        respond
            .take()
            .expect("structured response is sent once")
            .send(Ok(StructuredResultStream {
                columns,
                batches,
                operation_id: Some(active_id.clone()),
            }))
            .map_err(|_| EngineError::shutdown("Structured result receiver dropped"))?;
        stream_sender = Some(tx.clone());
        let mut count = 0;
        let mut batch = Vec::with_capacity(RESULT_BATCH_ROWS);
        while let Some(row) = rows
            .next()
            .map_err(|e| EngineError::duckdb(format!("Row error: {}", e)))?
        {
            ensure_result_row_limit(count)?;
            let mut values = Vec::with_capacity(col_count);
            for i in 0..col_count {
                values.push(duckdb_value_to_json(row, i));
            }
            batch.push(values);
            count += 1;
            if batch.len() == RESULT_BATCH_ROWS {
                tx.blocking_send(Ok(std::mem::replace(
                    &mut batch,
                    Vec::with_capacity(RESULT_BATCH_ROWS),
                )))
                .map_err(|_| EngineError::shutdown("Structured result receiver dropped"))?;
            }
        }
        if !batch.is_empty() {
            tx.blocking_send(Ok(batch))
                .map_err(|_| EngineError::shutdown("Structured result receiver dropped"))?;
        }
        Ok(())
    });
    *active_operation.lock().unwrap_or_else(|e| e.into_inner()) = None;
    operations
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .finish(active_id);
    if let Err(error) = result {
        if let Some(tx) = stream_sender {
            let _ = tx.blocking_send(Err(error));
        } else if let Some(respond) = respond {
            let _ = respond.send(Err(error));
        }
    }
}
