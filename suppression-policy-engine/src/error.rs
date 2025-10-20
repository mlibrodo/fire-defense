use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(&'static str),
    Conflict(&'static str),
    NotFound(&'static str),
}

#[derive(Serialize)]
struct ErrBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ErrBody {
                    error: msg.to_string(),
                }),
            )
                .into_response(),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(ErrBody {
                    error: msg.to_string(),
                }),
            )
                .into_response(),
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(ErrBody {
                    error: msg.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
