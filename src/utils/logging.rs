//! Logging utilities
//!
//! This module contains logging configuration and utilities.

use anyhow::Result;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Initialize the logging system
pub fn init_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Default log level configuration
            EnvFilter::new("crabfin=debug,info")
        });

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Logging system initialized");
    Ok(())
}

/// Initialize logging for development with more verbose output
pub fn init_dev_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("crabfin=trace,debug")
        });

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_ansi(true)
        .pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Development logging system initialized");
    Ok(())
}

/// Initialize logging for production with structured output
pub fn init_prod_logging() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("crabfin=info,warn,error")
        });

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::NONE)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Production logging system initialized");
    Ok(())
}

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level (least verbose)
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

impl From<LogLevel> for String {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => "trace".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Whether to enable colored output
    pub colored: bool,
    /// Whether to include file and line information
    pub include_location: bool,
    /// Whether to include thread information
    pub include_thread: bool,
    /// Whether to use JSON format
    pub json_format: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            colored: true,
            include_location: true,
            include_thread: false,
            json_format: false,
        }
    }
}

/// Initialize logging with custom configuration
pub fn init_logging_with_config(config: LoggingConfig) -> Result<()> {
    let level_str: String = config.level.into();
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new(format!("crabfin={}", level_str))
        });

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(config.include_thread)
        .with_thread_names(config.include_thread)
        .with_file(config.include_location)
        .with_line_number(config.include_location)
        .with_ansi(config.colored);

    // Note: JSON formatting not available in this version of tracing-subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Custom logging system initialized with config: {:?}", config);
    Ok(())
}

/// Macro for logging performance metrics
#[macro_export]
macro_rules! log_duration {
    ($name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        tracing::debug!("{} took {:?}", $name, duration);
        result
    }};
}

/// Macro for conditional debug logging
#[macro_export]
macro_rules! debug_if {
    ($condition:expr, $($arg:tt)*) => {
        if $condition {
            tracing::debug!($($arg)*);
        }
    };
}