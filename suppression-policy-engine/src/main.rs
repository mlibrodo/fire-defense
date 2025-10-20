mod engine;
mod policy;
mod web;

use axum::serve;
use std::sync::Arc;
use std::{env, net::SocketAddr};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    // state: share Engine via Arc
    let state = web::AppState {
        engine: Arc::new(engine::Engine::new()),
    };

    // routes
    let app = web::routes(state);

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8100);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();

    println!("listening on http://{}", addr);
    serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
