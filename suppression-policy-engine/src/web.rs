use crate::state::AppState;
use axum::{routing::get, Router};

use tower_http::trace::TraceLayer;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(crate::routes::index::handler))
        .merge(crate::routes::v1::router(state))
        .layer(TraceLayer::new_for_http())
}
