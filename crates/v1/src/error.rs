//! アプリケーション全体で使用するエラー型及び変換ロジックを集約するモジュール。

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

/// プロジェクト全体で使用するResult型。
pub type AppResult<T> = Result<T, AppError>;

/// PostgreSQLのSQLSTATEコード定数。
pub mod sqlx_error_code {
    pub const UNIQUE_VIOLATION: &str = "23505";
    pub const FK_VIOLATION: &str = "23503";
    pub const NOT_NULL_VIOLATION: &str = "23502";
    pub const CHECK_VIOLATION: &str = "23514";
}

/// アプリケーション全体で使用される上位エラー型。
/// 各バリアントは対応する<HTTP Status Code>とOpt.の<Detail>を持つ。
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
    /// AppErrorを<HTTP Status Code>に変換する。
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
    /// AppErrorが持つ<Detail>を返す（無ければ None）。
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
    /// AppErrorをaxumの<HTTP Response>に変換する。
    fn into_response(self) -> Response {
        let status = self.status_code();

        // ログ出力（500系はerror、それ以外はwarn）
        if status.is_server_error() {
            error!(?self, "internal server error");
        } else {
            warn!(?self, "client error");
        }

        // Statusに応じてResponse Bodyを構築（500系には<Detail>を含めない）
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
                message: status.canonical_reason().unwrap_or("Error").to_string(),
                detail: self.detail().cloned(),
                instance: None,
                timestamp: Utc::now().timestamp(),
            }
        };

        (status, Json(body)).into_response()
    }
}

/// sqlx のエラーをAppErrorに変換する。
impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        // エラーを文字列化する。
        let e_str = e.to_string();

        match e {
            SqlxError::RowNotFound => AppError::NotFound(Some("Resource not found".into())),
            SqlxError::PoolTimedOut => AppError::RequestTimeout(Some("Database timeout".into())),
            SqlxError::Database(db_err) => match db_err.code().unwrap_or_default().as_ref() {
                sqlx_error_code::UNIQUE_VIOLATION => {
                    AppError::Conflict(Some("Duplicate key".into()))
                }
                sqlx_error_code::FK_VIOLATION => {
                    AppError::Conflict(Some("Foreign-key violation".into()))
                }
                sqlx_error_code::NOT_NULL_VIOLATION => {
                    AppError::BadRequest(Some("Null value in column".into()))
                }
                sqlx_error_code::CHECK_VIOLATION => {
                    AppError::UnprocessableContent(Some("Check violation".into()))
                }
                code => AppError::InternalServerError(Some(format!(
                    "Database error ({code}): {}",
                    db_err.message()
                ))),
            },
            // 文字列に"timeout"が含まれていれば408エラー。
            _ if e_str.contains("timeout") => {
                AppError::RequestTimeout(Some("Database timeout".into()))
            }

            // その他不明なエラー。
            other => AppError::InternalServerError(Some(format!("DB error: {other}"))),
        }
    }
}

/// パスワードのハッシュ化・検証に関連するエラー。
#[derive(Debug, Error)]
pub enum HashingError {
    #[error("Password mismatch")]
    PasswordMismatch,
    #[error("Argon2 error: {0}")]
    Argon2(#[from] Argon2Error),
}

/// ドメイン層で使用されるデータベース関連のエラー。
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Row not found")]
    NotFound,
    #[error(transparent)]
    Sqlx(#[from] SqlxError),
}
