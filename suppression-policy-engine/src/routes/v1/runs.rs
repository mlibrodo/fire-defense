use axum::{
    extract::{Path, Query, State},
    http::{header::LOCATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::atomic::Ordering;

use crate::suppression_policy_runner::spawn_run; // if your path is `crate::spr::runner::spawn_run`, change this import accordingly
use crate::{
    engine::Evaluation,
    error::ApiError,
    idempotency::{IdemRecord, IdempotencyStore},
    models::{EvaluateRequest, RunRecord, RunStatus, StartRunRequest},
    state::AppState,
    time::now_ms,
};

/* -------------------- GET /v1/runs/{run_id} -------------------- */

pub async fn get_run(State(state): State<AppState>, Path(run_id): Path<String>) -> Response {
    let guard = state.runs.read().await;
    if let Some(r) = guard.get(&run_id) {
        return (StatusCode::OK, Json(r.clone())).into_response();
    }
    ApiError::NotFound("run not found").into_response()
}

/* -------------------- POST /v1/runs/{run_id}/cancel -------------------- */

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

/* -------------------- GET /api/evaluate --------------------
Used by index.html. Plans the policy.
If dry_run=false, it also creates a run and spawns the runner.
----------------------------------------------------------- */

pub async fn get_evaluate_query(
    State(state): State<AppState>,
    Query(q): Query<EvaluateRequest>,
) -> Response {
    // default installation for demo page, if not provided
    let installation_id = q
        .installation_id
        .clone()
        .unwrap_or_else(|| "house-123".to_string());

    // Always compute plan (no device actions in the engine now)
    let eval: Evaluation = state.engine.evaluate(&installation_id, q.policy, q.dry_run);

    if q.dry_run {
        // Preview only
        return (StatusCode::OK, Json(eval)).into_response();
    }

    // Execute path (create run and spawn runner) — mirror POST /v1/installations/{id}/runs
    let start = StartRunRequest {
        policy: q.policy,
        dry_run: q.dry_run,
        metadata: None,
        requested_by: Some("ui-evaluate".to_string()),
    };

    match create_run_and_response(state.clone(), installation_id, start).await {
        Ok(created) => (
            StatusCode::CREATED,
            [(LOCATION, format!("/v1/runs/{}", created.run_id))],
            Json(serde_json::json!({
                "run_id": created.run_id,
                "evaluation": eval  // includes policy, level, summary, dry_run
            })),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}

/* -------------------- POST /v1/installations/{installation_id}/runs --------------------
Canonical “start run” endpoint with Idempotency-Key support.
------------------------------------------------------------------ */

pub async fn post_start_run(
    State(state): State<AppState>,
    Path(installation_id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<StartRunRequest>,
) -> Response {
    // Idempotency fingerprint
    let body_json = serde_json::json!({
        "policy": body.policy,
        "dry_run": body.dry_run,
        "metadata": body.metadata,
        "requested_by": body.requested_by,
    });
    let fp = IdempotencyStore::fingerprint_json(&body_json);

    if let Some(key) = headers.get("Idempotency-Key").and_then(|v| v.to_str().ok()) {
        if key.trim().is_empty() {
            return ApiError::BadRequest("empty Idempotency-Key").into_response();
        }
        let scoped = state.idempotency.scope_key(&installation_id, key);

        // Replay if we’ve seen this exact request
        if let Some(rec) = state.idempotency.get(&scoped).await {
            if rec.body_fingerprint != fp {
                return ApiError::Conflict("Idempotency-Key reuse with different body")
                    .into_response();
            }
            return (
                StatusCode::OK,
                [
                    (LOCATION, format!("/v1/runs/{}", rec.run_id)),
                    (
                        axum::http::HeaderName::from_static("idempotency-replayed"),
                        "true".into(),
                    ),
                ],
                Json(rec.response),
            )
                .into_response();
        }

        // First time — create and cache
        match create_run_and_response(state.clone(), installation_id, body).await {
            Ok(created) => {
                state
                    .idempotency
                    .put(
                        scoped,
                        IdemRecord {
                            run_id: created.run_id.clone(),
                            body_fingerprint: fp,
                            response: created.response.clone(),
                            created_at_ms: now_ms(),
                        },
                    )
                    .await;

                return (
                    StatusCode::CREATED,
                    [(LOCATION, format!("/v1/runs/{}", created.run_id))],
                    Json(created.response),
                )
                    .into_response();
            }
            Err(e) => return e.into_response(),
        }
    }

    // No Idempotency-Key — normal create
    match create_run_and_response(state.clone(), installation_id, body).await {
        Ok(created) => (
            StatusCode::CREATED,
            [(LOCATION, format!("/v1/runs/{}", created.run_id))],
            Json(created.response),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}

/* -------------------- shared create helper -------------------- */

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
                // If your RunRecord still has an `actions` field, keep this line.
                // If you removed it from the model, delete this line.
                actions: Vec::new(),
                dry_run: body.dry_run,
                status: RunStatus::Starting,
                started_at_ms: now,
                updated_at_ms: now,
                cancel_requested: false,
            },
        );
    }

    // Run in background
    // NOTE: use your actual runner path and field name for the DAL.
    // If your state field is `dal`, this is correct:
    spawn_run(state.clone(), run_id.clone());

    let resp = serde_json::json!({
        "run_id": run_id,
        "installation_id": installation_id,
        "status": "starting",
        "policy": eval.policy,
        "level": eval.level,
        "summary": eval.summary,
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
