use crate::error::{AppError, AppResult};
use config::{Config, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;
use std::path::PathBuf;
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub postgres: Postgres,
    pub logging: Logging,
}

/// [app] section
#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub version: String,
    pub port: u16,
}

/// [postgres] section
#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
}

/// [logging] section
#[derive(Debug, Deserialize)]
pub struct Logging {
    /// Logging level. Allowed values: "error", "warn", "info", "debug", "trace"
    pub level: String,
    /// Logging format. Allowed values: "json", "plain"
    pub format: String,
}

impl AppConfig {
    /// Read defaults.toml → development.toml → environment variables in this order
    pub fn new() -> AppResult<Self> {
        // Read environment variables, but don't error if .env is missing
        if dotenv().is_err() {
            warn!(".env file not found or failed to load");
        }

        // Create an absolute path to workspace root/config
        let root_config = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent()) // workspace root
            .expect("workspace root")
            .join("config");

        let builder = Config::builder()
            .add_source(
                File::from(root_config.join("defaults.toml")).required(true),
            )
            .add_source(
                File::from(root_config.join("development.toml"))
                    .required(false),
            )
            .add_source(Environment::with_prefix("APP").separator("__"))
            .add_source(Environment::with_prefix("POSTGRES").separator("__"))
            .add_source(Environment::with_prefix("LOGGING").separator("__"));

        builder
            .build()
            .map_err(|e| {
                AppError::InternalServerError(Some(format!(
                    "Failed to build configuration from files and environment: {}",
                    e
                )))
            })?
            .try_deserialize()
            .map_err(|e| {
                AppError::InternalServerError(Some(format!(
                    "Failed to deserialize configuration into AppConfig struct: {}",
                    e
                )))
            })
    }

    /// postgres接続用URLを組立てて返す。
    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres.user,
            self.postgres.password,
            self.postgres.host,
            self.postgres.port,
            self.postgres.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    /// 環境変数やファイルから読み込んだAppConfigをDebugで表示する。
    #[test]
    fn print_app_config() {
        // 設定読み込み
        let cfg = AppConfig::new().expect("Failed to load AppConfig");

        // Debug 表現で出力（pretty print）
        println!("{:#?}", cfg);
    }
}
