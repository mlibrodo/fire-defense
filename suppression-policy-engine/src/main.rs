mod dal;
mod engine;
mod error;
mod idempotency;
mod models;
mod policy;
mod routes;
mod spr;
mod state;
mod telemetry;
mod time;
mod web;

use std::{env, net::SocketAddr, sync::Arc};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    let engine = Arc::new(engine::Engine::new());
    let dal = Arc::new(dal::drivers::MockDriver);
    let telemetry = Arc::new(telemetry::NoopSink);

    let state = crate::state::AppState::new(engine, dal, telemetry);
    let app = web::routes(state.clone());

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
