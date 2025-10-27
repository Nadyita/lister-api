use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ultimatelister_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting server on {}:{}", config.host, config.port);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    tracing::info!("Connected to database");

    // Log auth status
    if config.auth_token.is_some() {
        tracing::info!("Authentication enabled");
    } else {
        tracing::info!("Authentication disabled - API is open");
    }

    // Create application state
    let state = AppState {
        pool,
        config: config.clone(),
    };

    // Build application router
    let app = routes::create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .context("Failed to bind to address")?;

    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}

