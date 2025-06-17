use crate::error::{AppError, AppResult};
use config::{Config, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;
use urlencoding::encode;

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

impl Logging {
    /// LevelをtracingのLevelに変換して返す。
    pub fn level_filter(&self) -> LevelFilter {
        match self.level.to_lowercase().as_str() {
            "error" => LevelFilter::ERROR,
            "warn" => LevelFilter::WARN,
            "info" => LevelFilter::INFO,
            "debug" => LevelFilter::DEBUG,
            "trace" => LevelFilter::TRACE,
            // 設定値が上記に存在しない場合は，infoを返す。
            other => {
                warn!("Unknown log level '{}', defaulting to INFO", other);
                LevelFilter::INFO
            }
        }
    }

    /// ログのフォーマットがJSONか，それ以外(PRETTY)か判定する。
    /// JSONの場合は，Trueを返す。
    pub fn is_json(&self) -> bool {
        matches!(self.format.to_lowercase().as_str(), "json" | "structured")
    }
}

impl AppConfig {
    /// Read defaults.toml → development.toml → environment variables in this order
    pub fn new() -> AppResult<Self> {
        // Read environment variables, but don't error if .env is missing
        if dotenv().is_err() {
            warn!(".env file not found or failed to load");
        }

        let config_dir = Self::workspace_root()?;
        info!("Loading configuration from {:?}", config_dir);

        let builder = Config::builder()
            .add_source(File::from(config_dir.join("defaults.toml")).required(true))
            .add_source(File::from(config_dir.join("development.toml")).required(false))
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
            encode(&self.postgres.password), // passwordはエンコードする。
            self.postgres.host,
            self.postgres.port,
            self.postgres.name
        )
    }

    pub fn get_masked_postgres_url(&self) -> String {
        let masked_user = self.postgres.user.chars().next().unwrap_or('_');
        let masked_pass = "**".to_string();
        let masked_host = self.postgres.host.chars().next().unwrap_or('_');
        let masked_port = self.postgres.port.to_string().chars().next().unwrap_or('_'); //一桁目のみ
        let masked_name = self.postgres.name.chars().next().unwrap_or('_');
        format!(
            "postgres://{}*:{}@{}*:{}*/{}*",
            masked_user, masked_pass, masked_host, masked_port, masked_name
        )
    }

    fn workspace_root() -> AppResult<PathBuf> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir
            .parent() // crates
            .and_then(|p| p.parent()) // workspace root
            .ok_or_else(|| {
                AppError::InternalServerError(Some(
                    "Failed to locate workspace root from CARGO_MANIFEST_DIR".into(),
                ))
            })?;
        let config_dir = root.join("config");
        if !config_dir.is_dir() {
            return Err(AppError::InternalServerError(Some(format!(
                "Expected config directory at {:?}, but not found",
                config_dir
            ))));
        }
        Ok(config_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;
    /// AppConfig が正常に読み込めるか確認し、内容を表示
    #[test]
    fn print_app_config() {
        let cfg = AppConfig::new().expect("Failed to load AppConfig");
        println!("{:#?}", cfg);
    }
}
