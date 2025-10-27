mod device_abstraction_layer;
mod enactor;
mod engine;
mod error;
mod idempotency;
mod models;
mod policy;
mod routes;
mod state;
mod suppression_policy_runner;
mod telemetry;
mod time;
mod web;

use std::{collections::HashMap, env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{fmt, EnvFilter};

// ✅ correct imports for the resolver + drivers
use crate::device_abstraction_layer::drivers::control_by_web::{
    InMemoryResolver, InstallationAccountResolver,
};
use crate::device_abstraction_layer::drivers::{
    ControlByWebConfig, ControlByWebDriver, MockDriver,
};
use crate::device_abstraction_layer::DeviceDriver;

#[tokio::main]
async fn main() {
    // logging
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    let engine = Arc::new(engine::Engine::new());

    // ── Build installation → account resolver
    let mut map = HashMap::new();
    // Example mapping (replace with real data source)
    let default_account: Option<u64> = env::var("CBW_ACCOUNT_ID_DEFAULT")
        .ok()
        .and_then(|s| s.parse().ok());
    if let (Some(aid), Some(did)) = (
        env::var("CBW_ACCOUNT_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok()),
        env::var("CBW_DEVICE_ID").ok(),
    ) {
        map.insert("sebastians-house".to_string(), (aid, did));
    }

    let default_device = env::var("CBW_DEVICE_ID_DEFAULT").ok();
    let resolver: Arc<dyn InstallationAccountResolver> =
        Arc::new(InMemoryResolver::new(map, default_account, default_device));
    // ── Build DAL (prefer ControlByWeb; fall back to Mock on any error)
    let device_abstraction_layer: Arc<dyn DeviceDriver> = match ControlByWebConfig::from_env() {
        Ok(cfg) => match ControlByWebDriver::new(cfg, resolver.clone()) {
            Ok(driver) => Arc::new(driver),
            Err(e) => {
                eprintln!("CBW driver init failed: {e}; falling back to MockDriver");
                Arc::new(MockDriver)
            }
        },
        Err(_) => Arc::new(MockDriver),
    };

    let telemetry = Arc::new(telemetry::NoopSink);

    let app_state = state::AppState::new(engine, device_abstraction_layer, telemetry);
    let app = web::routes(app_state.clone());

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8100);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
