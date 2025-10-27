use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc},
};
use tokio::sync::RwLock;

use crate::enactor::InstallationEnactor;
use crate::{
    device_abstraction_layer::DeviceDriver, engine::Engine, idempotency::IdempotencyStore,
    models::RunRecord, telemetry::TelemetrySink,
};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Engine>,
    pub runs: Arc<RwLock<HashMap<String, RunRecord>>>,
    pub run_counter: Arc<AtomicU64>,
    pub idempotency: Arc<IdempotencyStore>,
    pub device_abstraction_layer: Arc<dyn DeviceDriver>,
    pub enactor: Arc<dyn InstallationEnactor>,
    pub telemetry: Arc<dyn TelemetrySink>,
}

impl AppState {
    pub fn new(
        engine: Arc<Engine>,
        device_abstraction_layer: Arc<dyn DeviceDriver>,
        telemetry: Arc<dyn TelemetrySink>,
    ) -> Self {
        let enactor = Arc::new(crate::enactor::SimpleEnactor::new(
            device_abstraction_layer.clone(),
        )) as Arc<dyn InstallationEnactor>;
        let idempotency = Arc::new(IdempotencyStore::new());
        Self {
            engine,
            runs: Arc::new(RwLock::new(HashMap::new())),
            run_counter: Arc::new(AtomicU64::new(1)),
            idempotency,
            device_abstraction_layer,
            enactor,
            telemetry,
        }
    }
}
