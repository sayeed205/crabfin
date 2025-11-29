//! Crabfin - Native Client
//!
//! A modern, native desktop application for accessing media servers
//! built with Rust and GPUI.

use anyhow::Result;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod client;
mod auth;
mod ui;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for structured logging
    init_logging()?;

    info!("Starting Crabfin - Native Client");

    // Run the main application
    match run_app().await {
        Ok(_) => {
            info!("Application shutdown successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application error: {:#}", e);
            Err(e)
        }
    }
}

/// Initialize the tracing subscriber for logging
fn init_logging() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crabfin=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logging: {}", e))?;

    Ok(())
}

/// Main application entry point
async fn run_app() -> Result<()> {
    info!("Initializing application components");

    // Run the GPUI application
    app::run_crabfin_app().await?;

    info!("Application shutdown successfully");
    Ok(())
}