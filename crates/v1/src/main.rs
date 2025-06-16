use axum::{Router, extract::Extension, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, SocketAddr};
use tokio::{net::TcpListener, signal};
use tracing::{Level, info};
use tracing_subscriber;
use v1::{
    config::AppConfig,
    error::{AppError, AppResult},
};

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Configの読み込み
    let config = AppConfig::new()?;
    let postgres_url = config.get_postgres_url();
    let postgres_pool = PgPoolOptions::new()
        .connect(&postgres_url)
        .await
        .map_err(|e| {
            AppError::InternalServerError(Some(format!("Failed to connect with postgres: {}", e)))
        })?;
    info!("Connected to the postgres!");

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

#[test]
fn debug() {
    let config = AppConfig::new().expect("Failed to create AppConfig");
    let postgres_url = config.get_postgres_url();
    println!("{}", postgres_url);
}
