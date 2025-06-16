//! Helpers for successful API responses.

use crate::presentation::common_dto::ApiResponse;
use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::Serialize;

/// Wraps any serializable payload into a unified success envelope.
pub fn api_ok<T: Serialize>(data: T) -> impl IntoResponse {
    let body = ApiResponse {
        data,
        message: "success".into(),
        timestamp: Utc::now().timestamp(),
    };
    (StatusCode::OK, Json(body))
}
