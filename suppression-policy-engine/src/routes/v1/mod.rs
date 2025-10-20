use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub mod health;
pub mod installations;
pub mod runs;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health::get))
        .route(
            "/v1/installations/{installation_id}/runs",
            post(installations::post_start_run),
        )
        .route("/v1/runs/{run_id}", get(runs::get_run))
        .route("/v1/runs/{run_id}/cancel", post(runs::post_cancel_run))
        // ✅ Keep this for the HTML form
        .route("/api/evaluate", get(installations::get_evaluate_query))
        .with_state(state)
}
