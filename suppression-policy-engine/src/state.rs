use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc},
};
use tokio::sync::RwLock;

use crate::{
    dal::DeviceDriver, engine::Engine, idempotency::IdempotencyStore, models::RunRecord,
    telemetry::TelemetrySink,
};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Engine>,
    pub runs: Arc<RwLock<HashMap<String, RunRecord>>>,
    pub run_counter: Arc<AtomicU64>,
    pub idempotency: IdempotencyStore,
    pub dal: Arc<dyn DeviceDriver>,
    pub telemetry: Arc<dyn TelemetrySink>,
}

impl AppState {
    pub fn new(
        engine: Arc<Engine>,
        dal: Arc<dyn DeviceDriver>,
        telemetry: Arc<dyn TelemetrySink>,
    ) -> Self {
        Self {
            engine,
            runs: Arc::new(RwLock::new(HashMap::new())),
            run_counter: Arc::new(AtomicU64::new(1)),
            idempotency: IdempotencyStore::new(),
            dal,
            telemetry,
        }
    }
}
