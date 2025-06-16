//! Centralized application errors and conversions.
use crate::presentation::common_dto::ApiError;
use AppError::*;
use argon2::password_hash::Error as Argon2Error;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use sqlx::Error as SqlxError;
use thiserror::Error;
use tracing::*;

/// Convenient alias used across the project.
pub type AppResult<T> = Result<T, AppError>;

/// Top-level application error.
/// Each variant maps to an HTTP status code and optional detail.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad Request")]
    BadRequest(Option<String>),
    #[error("Unauthorized")]
    Unauthorized(Option<String>),
    #[error("Forbidden")]
    Forbidden(Option<String>),
    #[error("Not Found")]
    NotFound(Option<String>),
    #[error("Request Timeout")]
    RequestTimeout(Option<String>),
    #[error("Conflict")]
    Conflict(Option<String>),
    #[error("I'm a Teapot")]
    ImATeapot(Option<String>),
    /// validation error
    #[error("Unprocessable Content")]
    UnprocessableContent(Option<String>),
    #[error("Internal Server Error")]
    InternalServerError(Option<String>),
}

impl AppError {
    /// Converted to HTTP status code
    pub fn status_code(&self) -> StatusCode {
        use AppError::*;
        match self {
            BadRequest(_) => StatusCode::BAD_REQUEST,
            Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Forbidden(_) => StatusCode::FORBIDDEN,
            NotFound(_) => StatusCode::NOT_FOUND,
            RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            Conflict(_) => StatusCode::CONFLICT,
            ImATeapot(_) => StatusCode::IM_A_TEAPOT,
            UnprocessableContent(_) => StatusCode::UNPROCESSABLE_ENTITY,
            InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    /// optional detail
    pub fn detail(&self) -> Option<&String> {
        match self {
            BadRequest(d)
            | Unauthorized(d)
            | Forbidden(d)
            | NotFound(d)
            | RequestTimeout(d)
            | Conflict(d)
            | ImATeapot(d)
            | UnprocessableContent(d)
            | InternalServerError(d) => d.as_ref(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // Always log errors for consistency
        if status.is_server_error() {
            error!(?self, "internal server error");
        } else {
            warn!(?self, "client error");
        }

        // 5xx does not return detail
        let body = if status.is_server_error() {
            ApiError {
                status: status.as_u16(),
                message: status
                    .canonical_reason()
                    .unwrap_or("Internal Server Error")
                    .to_string(),
                detail: None,
                instance: None,
                timestamp: Utc::now().timestamp(),
            }
        } else {
            ApiError {
                status: status.as_u16(),
                message: status
                    .canonical_reason()
                    .unwrap_or("Error")
                    .to_string(),
                detail: self.detail().cloned(),
                instance: None,
                timestamp: Utc::now().timestamp(),
            }
        };

        (status, Json(body)).into_response()
    }
}

// Conversions
impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::RowNotFound => {
                AppError::NotFound(Some("Resource not found".to_string()))
            }
            other => AppError::InternalServerError(Some(format!(
                "DB error: {other}"
            ))),
        }
    }
}

/// Password hashing / verification errors.
#[derive(Debug, Error)]
pub enum HashingError {
    #[error("Password mismatch")]
    PasswordMismatch,
    #[error("Argon2 error: {0}")]
    Argon2(#[from] Argon2Error),
}

/// Database errors used in domain layer.
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Row not found")]
    NotFound,
    #[error(transparent)]
    Sqlx(#[from] SqlxError),
}
