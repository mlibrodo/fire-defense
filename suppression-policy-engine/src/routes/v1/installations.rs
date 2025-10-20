use axum::http::HeaderMap;
use axum::{
    extract::{Path, Query, State},
    http::{header::LOCATION, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::models::EvaluateRequest;
use crate::{
    engine::Evaluation,
    error::ApiError,
    idempotency::{IdemRecord, IdempotencyStore},
    models::StartRunRequest,
    state::AppState,
    time::now_ms,
};

/// GET /api/evaluate – used by index.html
pub async fn get_evaluate_query(
    State(state): State<AppState>,
    Query(q): Query<EvaluateRequest>,
) -> Json<Evaluation> {
    let installation_id = q.installation_id.unwrap_or_else(|| "house-123".to_string());
    let out = state.engine.evaluate(&installation_id, q.policy, q.dry_run);
    Json(out)
}
pub async fn post_start_run(
    State(state): State<AppState>,
    Path(installation_id): Path<String>,
    headers: HeaderMap,
    Json(body): Json<StartRunRequest>,
) -> Response {
    // fingerprint body
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

        if let Some(rec) = state.idempotency.get(&scoped).await {
            if rec.body_fingerprint != fp {
                return ApiError::Conflict("Idempotency-Key reuse with different body")
                    .into_response();
            }
            // replay
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

        // create + cache
        match super::runs::create_run_and_response(state.clone(), installation_id, body).await {
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

    // no idempotency key → normal create
    match super::runs::create_run_and_response(state.clone(), installation_id, body).await {
        Ok(created) => (
            StatusCode::CREATED,
            [(LOCATION, format!("/v1/runs/{}", created.run_id))],
            Json(created.response),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}
