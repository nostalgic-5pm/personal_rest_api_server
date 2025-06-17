/// Defines the standard format for API responses.
use serde::Serialize;

/// Successful response structure.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// The actual response data.
    pub data: T,
    /// A message describing the result or providing additional context.
    pub message: String,
    /// The time the response was generated (UNIX timestamp).
    pub timestamp: i64,
}

/// Error response structure.
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// HTTP status code corresponding to the error.
    pub status: u16,
    /// A short, human-readable summary of the error.
    pub message: String,
    /// An optional detailed explanation of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// An optional URI or identifier of the instance where the error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// The time the error response was generated (UNIX timestamp).
    pub timestamp: i64,
}
