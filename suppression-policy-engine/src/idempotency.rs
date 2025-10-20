use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A stored result for an idempotent create call.
#[derive(Clone, Debug, Serialize)]
pub struct IdemRecord {
    pub run_id: String,
    pub body_fingerprint: u64,
    pub response: Value,
    pub created_at_ms: u128,
}

/// In-memory idempotency store.
/// You can swap this out later for Redis/Postgres with the same API.
#[derive(Clone, Default)]
pub struct IdempotencyStore {
    inner: Arc<RwLock<HashMap<String, IdemRecord>>>,
}

impl IdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Scope the key (optional): keeps keys unique per installation.
    pub fn scope_key(&self, installation_id: &str, key: &str) -> String {
        format!("{}::{}", installation_id, key)
    }

    /// Canonical-ish fingerprint for a JSON body (good enough for in-memory).
    pub fn fingerprint_json(value: &Value) -> u64 {
        let bytes = serde_json::to_vec(value).unwrap_or_default();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        hasher.finish()
    }

    /// Get a stored record (clone) if present.
    pub async fn get(&self, scoped_key: &str) -> Option<IdemRecord> {
        let map = self.inner.read().await;
        map.get(scoped_key).cloned()
    }

    /// Insert/overwrite a record.
    pub async fn put(&self, scoped_key: String, rec: IdemRecord) {
        let mut map = self.inner.write().await;
        map.insert(scoped_key, rec);
    }

    /// Optional: simple TTL sweeping hook (no-op for now).
    #[allow(dead_code)]
    pub async fn sweep_older_than(&self, _now_ms: u128, _ttl_ms: u128) {
        // implement if/when needed
    }
}
