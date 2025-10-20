use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::atomic::Ordering;

use crate::spr::spawn_run;
use crate::{
    error::ApiError,
    models::{RunRecord, RunStatus, StartRunRequest},
    state::AppState,
    time::now_ms,
};

pub async fn get_run(State(state): State<AppState>, Path(run_id): Path<String>) -> Response {
    let guard = state.runs.read().await;
    if let Some(r) = guard.get(&run_id) {
        return (StatusCode::OK, Json(r.clone())).into_response();
    }
    ApiError::NotFound("run not found").into_response()
}

pub async fn post_cancel_run(
    State(state): State<AppState>,
    Path(run_id): Path<String>,
) -> Response {
    let mut guard = state.runs.write().await;
    if let Some(r) = guard.get_mut(&run_id) {
        if matches!(
            r.status,
            RunStatus::Succeeded | RunStatus::Failed | RunStatus::Canceled
        ) {
            return ApiError::Conflict("run already finished").into_response();
        }
        r.cancel_requested = true;
        if matches!(r.status, RunStatus::Starting | RunStatus::Running) {
            r.status = RunStatus::Canceling;
            r.updated_at_ms = now_ms();
        }
        return (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({"run_id": run_id, "status":"canceling"})),
        )
            .into_response();
    }
    ApiError::NotFound("run not found").into_response()
}

/* -------- shared create helper -------- */

pub struct CreatedRun {
    pub run_id: String,
    pub response: serde_json::Value,
}

pub async fn create_run_and_response(
    state: AppState,
    installation_id: String,
    body: StartRunRequest,
) -> Result<CreatedRun, ApiError> {
    let id_num = state.run_counter.fetch_add(1, Ordering::SeqCst);
    let run_id = format!("r_{id_num:016x}");
    let now = now_ms();

    let eval = state
        .engine
        .evaluate(&installation_id, body.policy, body.dry_run);

    {
        let mut guard = state.runs.write().await;
        guard.insert(
            run_id.clone(),
            RunRecord {
                run_id: run_id.clone(),
                installation_id: installation_id.clone(),
                policy: body.policy,
                level: eval.level,
                actions: eval.actions.clone(),
                dry_run: body.dry_run,
                status: RunStatus::Starting,
                started_at_ms: now,
                updated_at_ms: now,
                cancel_requested: false,
            },
        );
    }

    spawn_run(state.runs.clone(), state.dal.clone(), run_id.clone());

    let resp = serde_json::json!({
        "run_id": run_id,
        "installation_id": installation_id,
        "status": "starting",
        "policy": eval.policy,
        "level": eval.level,
        "actions": eval.actions,
    });

    let rid = resp
        .get("run_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    Ok(CreatedRun {
        run_id: rid,
        response: resp,
    })
}
