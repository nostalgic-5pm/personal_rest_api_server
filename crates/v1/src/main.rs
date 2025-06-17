use axum::{Router, extract::Extension, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, SocketAddr};
use tokio::{net::TcpListener, signal};
use tracing::info;
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use v1::{
    config::{AppConfig, Logging},
    error::{AppError, AppResult},
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // Configを読み込む
    let config = AppConfig::new()?;
    // Tracingの初期化
    init_tracing(&config.logging);
    info!("Configuration loaded: version {}", config.app.version);

    // postgres接続
    let postgres_url = config.get_postgres_url();
    let postgres_pool = PgPoolOptions::new()
        .connect(&postgres_url)
        .await
        .map_err(|e| {
            AppError::InternalServerError(Some(format!("Failed to connect with postgres: {}", e)))
        })?;
    info!(
        "Connected to the postgres: {}",
        config.get_masked_postgres_url()
    );

    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(postgres_pool));

    // Construct a socket address by combining host and port
    let ip: IpAddr =
        config.app.host.parse().map_err(|e| {
            AppError::InternalServerError(format!("Invalid IP address: {}", e).into())
        })?;
    let address = SocketAddr::new(ip, config.app.port);

    let listener = TcpListener::bind(&address)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to bind: {}", e).into()))?;
    info!("▶ Server running on http://{}", &address);

    // Start the Axum server with graceful shutdown
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            AppError::InternalServerError(format!("Failed to start application: {}", e).into())
        })?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler.");
    info!("Shutting down the server...")
}

fn init_tracing(config: &Logging) {
    // filter = Configで設定されているLoggingのレベル。
    let filter = config.level_filter();

    // ログのフォーマットを定義する。
    let fmt_layer = fmt::layer()
        .with_timer(UtcTime::rfc_3339())
        .with_level(true)
        .with_target(false)
        //.with_thread_ids(true)
        //.with_thread_names(true)
        ;

    // Json or Prettyでフォーマットする。
    if config.is_json() {
        tracing_subscriber::registry()
            .with(fmt_layer.json())
            .with(filter)
            .init()
    } else {
        tracing_subscriber::registry()
            .with(fmt_layer.pretty())
            .with(filter)
            .init()
    }
}

#[test]
fn debug() {
    let config = AppConfig::new().expect("Failed to create AppConfig");
    let postgres_url = config.get_postgres_url();
    println!("{}", postgres_url);
}
