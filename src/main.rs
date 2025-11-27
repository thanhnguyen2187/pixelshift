mod app_state;
mod err;
mod global;
mod handlers;

use crate::global::{CACHE_ITEM_MIN_SECONDS, CACHE_TOTAL_MAX_BYTES, HOST, PORT};
use app_state::AppState;
use axum::{Router, routing::get, routing::post};
use dotenvy::dotenv;
use err::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(RwLock::new(AppState::new()));
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/v1/convert-url", post(handlers::convert_url))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", *HOST, *PORT)).await?;

    info!("Server running on http://{}:{}", *HOST, *PORT);
    info!(
        "A cached item would exists for at least {} second(s)",
        *CACHE_ITEM_MIN_SECONDS
    );
    info!(
        "All cached items size would not exceed {} byte(s)/{} mb(s)",
        *CACHE_TOTAL_MAX_BYTES,
        *CACHE_TOTAL_MAX_BYTES / (1024 * 1024),
    );

    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
