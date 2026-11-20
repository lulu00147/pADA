use crate::engine::actor::EngineActor;
use crate::engine::EngineError;
use crate::security::AstInspector;
use crate::utils::QueryCache;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
fn new_operation_id() -> String {
    static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);
    format!(
        "timeout-{}",
        NEXT_OPERATION_ID.fetch_add(1, Ordering::Relaxed)
    )
}
use std::time::Duration;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
pub struct TimedActor {
    inner: Arc<EngineActor>,
    default_timeout: Duration,
    spatial_timeout: Duration,
    query_cache: Mutex<QueryCache>,
    semaphore: Arc<Semaphore>,
}
impl TimedActor {
    fn locked_cache(&self) -> std::sync::MutexGuard<'_, QueryCache> {
        self.query_cache.lock().unwrap_or_else(|e| e.into_inner())
    }
}
impl TimedActor {
    pub fn new(
        actor: EngineActor,
        default_timeout_secs: u64,
        spatial_timeout_secs: u64,
        max_concurrent: u32,
        cache_capacity: usize,
    ) -> Self {
        let max_concurrent = max_concurrent.max(1);
        Self {
            inner: Arc::new(actor),
            default_timeout: Duration::from_secs(default_timeout_secs),
            spatial_timeout: Duration::from_secs(spatial_timeout_secs),
            query_cache: Mutex::new(QueryCache::new(cache_capacity)),
            semaphore: Arc::new(Semaphore::new(max_concurrent as usize)),
        }
    }
    async fn acquire_permit(&self) -> Result<OwnedSemaphorePermit, EngineError> {
        tokio::time::timeout(self.default_timeout, self.semaphore.clone().acquire_owned())
            .await
            .map_err(|_| EngineError::timeout(
                "Timed out waiting for a query slot -- all concurrent slots are occupied. Try reducing parallelism."
            ))?
            .map_err(|e| EngineError::shutdown(e.to_string()))
    }
    async fn exec_with_timeout<F, T>(
        &self,
        timeout: Duration,
        label: &str,
        operation_id: &str,
        future: F,
    ) -> Result<T, EngineError>
    where
        F: Future<Output = Result<T, EngineError>>,
    {
        tokio::time::timeout(timeout, future).await.map_err(|_| {
            self.inner.interrupt(operation_id);
            EngineError::timeout(format!("{} timed out after {}s", label, timeout.as_secs()))
        })?
    }
    async fn wait_for_operation(&self, operation_id: &str) {
        while !matches!(
            self.inner.operation_state(operation_id),
            Some(crate::engine::types::OperationState::Finished) | None
        ) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    fn clear_query_cache(&self) {
        let mut cache = self.locked_cache();
        cache.clear();
    }
    pub async fn read(&self, sql: &str, query_id: Option<&str>) -> Result<String, EngineError> {
        {
            let mut cache = self.locked_cache();
            if let Some(cached) = cache.get(sql) {
                tracing::debug!("Query cache hit: {}", sql);
                return Ok(cached.to_string());
            }
        }
        let _permit = self.acquire_permit().await?;
        let timeout = self.detect_timeout(sql);
        let sql_owned = sql.to_string();
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                timeout,
                "Query",
                &operation_id,
                self.inner.read(sql_owned, Some(operation_id.clone())),
            )
            .await;
        if result.is_err() {
            self.wait_for_operation(&operation_id).await;
        }
        if let Ok(ref res) = result {
            let mut cache = self.locked_cache();
            cache.insert(sql, res.clone());
        }
        result
    }
    pub async fn validate_native(
        &self,
        sql: &str,
        query_id: Option<&str>,
    ) -> Result<(), EngineError> {
        let _permit = self.acquire_permit().await?;
        let timeout = self.detect_timeout(sql);
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                timeout,
                "Native SQL validation",
                &operation_id,
                self.inner
                    .validate_native(sql.to_string(), Some(operation_id.clone())),
            )
            .await;
        if result.is_err() {
            self.wait_for_operation(&operation_id).await;
        }
        result
    }
    pub async fn validate_native_on_write(
        &self,
        sql: &str,
        query_id: Option<&str>,
    ) -> Result<(), EngineError> {
        let _permit = self.acquire_permit().await?;
        let timeout = self.detect_timeout(sql);
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                timeout,
                "Pipeline native SQL validation",
                &operation_id,
                self.inner
                    .validate_native_on_write(sql.to_string(), Some(operation_id.clone())),
            )
            .await;
        if result.is_err() {
            self.wait_for_operation(&operation_id).await;
        }
        result
    }
    pub async fn read_structured_stream(
        &self,
        sql: &str,
        query_id: Option<&str>,
    ) -> Result<crate::engine::types::StructuredResultStream, EngineError> {
        let permit = self.acquire_permit().await?;
        let timeout = self.detect_timeout(sql);
        let sql_owned = sql.to_string();
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let deadline = tokio::time::Instant::now() + timeout;
        let mut stream = match self
            .exec_with_timeout(
                timeout,
                "Query",
                &operation_id,
                self.inner
                    .read_structured(sql_owned, Some(operation_id.clone())),
            )
            .await
        {
            Ok(stream) => stream,
            Err(error) => {
                while !matches!(
                    self.inner.operation_state(&operation_id),
                    Some(crate::engine::types::OperationState::Finished) | None
                ) {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                return Err(error);
            }
        };
        let inner = self.inner.clone();
        let timeout_operation_id = operation_id.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tokio::time::sleep_until(deadline) => {
                        inner.interrupt(&timeout_operation_id);
                        while !matches!(
                            inner.operation_state(&timeout_operation_id),
                            Some(crate::engine::types::OperationState::Finished) | None
                        ) {
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if matches!(
                            inner.operation_state(&timeout_operation_id),
                            Some(crate::engine::types::OperationState::Finished) | None
                        ) {
                            break;
                        }
                    }
                }
            }
            drop(permit);
        });
        stream.operation_id = Some(operation_id.clone());
        Ok(stream)
    }
    pub async fn read_structured_stream_on_write(
        &self,
        sql: &str,
        query_id: Option<&str>,
    ) -> Result<crate::engine::types::StructuredResultStream, EngineError> {
        let _permit = self.acquire_permit().await?;
        let timeout = self.detect_timeout(sql);
        let sql_owned = sql.to_string();
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                timeout,
                "Pipeline query",
                &operation_id,
                self.inner
                    .read_structured_on_write(sql_owned, Some(operation_id.clone())),
            )
            .await;
        if result.is_err() {
            self.wait_for_operation(&operation_id).await;
        }
        result
    }
    pub async fn write(&self, sql: &str, query_id: Option<&str>) -> Result<String, EngineError> {
        let _permit = self.acquire_permit().await?;
        let sql = sql.to_string();
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                self.default_timeout,
                "Write query",
                &operation_id,
                self.inner.write(sql, Some(operation_id.clone())),
            )
            .await;
        if result.is_err() {
            self.wait_for_operation(&operation_id).await;
        }
        if result.is_ok() {
            self.clear_query_cache();
            tracing::debug!("Query cache cleared after write");
        }
        result
    }
    pub async fn register_view(
        &self,
        view_name: &str,
        sql: &str,
        query_id: Option<&str>,
    ) -> Result<(), EngineError> {
        let _permit = self.acquire_permit().await?;
        let name = view_name.to_string();
        let stmt = sql.to_string();
        let operation_id = query_id
            .map(ToOwned::to_owned)
            .unwrap_or_else(new_operation_id);
        let result = self
            .exec_with_timeout(
                self.default_timeout,
                "View registration",
                &operation_id,
                self.inner
                    .register_view(name, stmt, Some(operation_id.clone())),
            )
            .await;
        if result.is_ok() {
            self.clear_query_cache();
            tracing::debug!("Query cache cleared after view registration");
        }
        result
    }
    pub fn interrupt(&self, query_id: &str) -> bool {
        self.inner.interrupt(query_id)
    }
    pub fn operation_state(&self, query_id: &str) -> Option<crate::engine::types::OperationState> {
        self.inner.operation_state(query_id)
    }
    pub fn note_temp_table_created(&self, name: &str) {
        self.inner.note_temp_table_created(name);
    }
    pub fn note_temp_table_dropped(&self, name: &str) {
        self.inner.note_temp_table_dropped(name);
    }
    pub fn expired_temp_tables(&self, ttl: Duration) -> Vec<(String, std::time::Instant)> {
        self.inner.expired_temp_tables(ttl)
    }
    pub fn temp_table_is_current(&self, name: &str, created: std::time::Instant) -> bool {
        self.inner.temp_table_is_current(name, created)
    }
    pub fn shutdown(&self) {
        self.inner.shutdown();
    }
    pub fn detect_timeout(&self, sql: &str) -> Duration {
        if AstInspector::contains_spatial_functions(sql) {
            tracing::debug!(
                "Spatial query detected, using extended timeout ({}s)",
                self.spatial_timeout.as_secs()
            );
            self.spatial_timeout
        } else {
            self.default_timeout
        }
    }
    pub fn is_available(&self) -> bool {
        self.inner.is_available()
    }
    pub fn cache_stats(&self) -> serde_json::Value {
        let cache = self.locked_cache();
        cache.stats()
    }
}
