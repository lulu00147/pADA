use crate::utils::cache_policy::is_cacheable as policy_is_cacheable;
use crate::utils::sql_normalize::normalize;
use indexmap::IndexMap;
use std::time::{Duration, Instant};
const CACHE_TTL: Duration = Duration::from_secs(30);
pub struct QueryCache {
    capacity: usize,
    map: IndexMap<String, (String, Instant)>,
}
fn is_cacheable(sql: &str) -> bool {
    policy_is_cacheable(sql)
}
impl QueryCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: IndexMap::new(),
        }
    }
    pub fn get(&mut self, sql: &str) -> Option<&str> {
        if !is_cacheable(sql) {
            return None;
        }
        let key = normalize(sql);
        let expired = self
            .map
            .get(&key)
            .is_some_and(|(_, t)| t.elapsed() >= CACHE_TTL);
        if expired {
            self.map.shift_remove(&key);
            return None;
        }
        if let Some(idx) = self.map.get_index_of(&key) {
            let last = self.map.len().saturating_sub(1);
            self.map.move_index(idx, last);
            self.map.get(&key).map(|(result, _)| result.as_str())
        } else {
            None
        }
    }
    pub fn insert(&mut self, sql: &str, result: String) {
        if self.capacity == 0 {
            return;
        }
        if !is_cacheable(sql) {
            return;
        }
        self.prune_expired();
        let key = normalize(sql);
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), (result, Instant::now()));
            if let Some(idx) = self.map.get_index_of(&key) {
                let last = self.map.len().saturating_sub(1);
                self.map.move_index(idx, last);
            }
            return;
        }
        if self.map.len() >= self.capacity {
            self.map.shift_remove_index(0);
        }
        if self.map.len() < self.capacity {
            self.map.insert(key, (result, Instant::now()));
        }
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    fn prune_expired(&mut self) {
        let now = Instant::now();
        self.map
            .retain(|_, (_, inserted_at)| now.duration_since(*inserted_at) < CACHE_TTL);
    }
    pub fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "total_entries": self.map.len(),
            "max_capacity": self.capacity,
        })
    }
}
