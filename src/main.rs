use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod client;
mod auth;
mod ui;
mod models;
mod services;
mod utils;

use app::JellyfinApp;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crabfin=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Crabfin - Jellyfin Native Client");

    // Run the application with proper error handling
    match JellyfinApp::run().await {
        Ok(()) => {
            info!("Application exited successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application error: {:#}", e);
            Err(e)
        }
    }
}