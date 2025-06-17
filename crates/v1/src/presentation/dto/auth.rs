use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthResponse {
    pub public_id: String,
    pub session_id: String,
    pub randomart: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterRequest {
    pub user_name: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterResponse {
    pub public_id: String,
    pub randomart: String,
}
