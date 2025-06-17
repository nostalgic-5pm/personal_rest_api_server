//! Helpers for successful API responses.

use crate::presentation::dto::common_dto::ApiResponse;
use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::Serialize;

/// Wraps any serializable payload into a unified success envelope.
pub fn api_ok<T: Serialize>(data: T, message: Option<&str>) -> impl IntoResponse {
    let message = message.unwrap_or("success").to_string();
    let body = ApiResponse {
        data,
        message,
        timestamp: Utc::now().timestamp(),
    };
    (StatusCode::OK, Json(body))
}
