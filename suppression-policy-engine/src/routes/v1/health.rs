use axum::Json;

pub async fn get() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}
