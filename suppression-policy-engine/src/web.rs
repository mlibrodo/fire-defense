use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    engine::{Engine, Evaluation},
    policy::Policy,
};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Engine>,
}

#[derive(Deserialize)]
pub struct EvalParams {
    pub installation_id: String,
    pub policy: Policy,
    #[serde(default)]
    pub dry_run: bool,
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/health", get(health))
        .route("/api/evaluate", get(api_evaluate))
        .with_state(state)
}

async fn index() -> Html<String> {
    // load templates at compile time
    static INDEX_HTML: &str = include_str!("../template/index.html");
    static TABLE_HTML: &str = include_str!("../template/policy_table.html");
    static ROW_HTML: &str = include_str!("../template/policy_row.html");

    // build rows from enum data (no HTML literals here)
    let rows = render_policy_rows(ROW_HTML);

    // insert rows into table
    let table = TABLE_HTML.replace("{{rows}}", &rows);

    // insert table into page
    let page = INDEX_HTML.replace("{{policy_table}}", &table);

    Html(page)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn api_evaluate(
    State(state): State<AppState>,
    Query(params): Query<EvalParams>,
) -> Json<Evaluation> {
    let out = state
        .engine
        .evaluate(&params.installation_id, params.policy, params.dry_run);
    Json(out)
}

// helper: render all policy rows using the provided row template
fn render_policy_rows(row_tpl: &str) -> String {
    let policies = [
        Policy::Observe,
        Policy::Prepare,
        Policy::Defend,
        Policy::Contain,
        Policy::Suppress,
    ];

    let mut out = String::new();
    for p in policies {
        let row = row_tpl
            .replace("{{name}}", &p.to_string())
            .replace("{{level}}", &p.level().to_string())
            .replace("{{summary}}", p.summary());
        out.push_str(&row);
    }
    out
}
