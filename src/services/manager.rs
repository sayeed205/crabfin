//! Service manager for coordinating business logic services
//!
//! This module provides a centralized service manager that coordinates
//! different business logic services throughout the application.

use gpui::Global;
use std::sync::Arc;

use crate::models::AppConfig;

/// Central service manager that coordinates all business logic services
#[derive(Clone)]
pub struct ServiceManager {
    /// Application configuration
    config: Arc<AppConfig>,
    /// Library service for media management
    library_service: Option<Arc<crate::services::LibraryService>>,
    /// Playback service for media playback
    playback_service: Option<Arc<crate::services::PlaybackService>>,
    /// Settings service for configuration management
    settings_service: Option<Arc<crate::services::SettingsService>>,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            library_service: None,
            playback_service: None,
            settings_service: None,
        }
    }

    /// Initialize all services
    pub fn initialize(&mut self) -> anyhow::Result<()> {
        tracing::info!("Initializing service manager");

        // Initialize settings service first as other services may depend on it
        self.settings_service = Some(Arc::new(
            crate::services::SettingsService::new(self.config.clone())?
        ));

        // Initialize library service
        self.library_service = Some(Arc::new(
            crate::services::LibraryService::new(self.config.clone())?
        ));

        // Initialize playback service
        self.playback_service = Some(Arc::new(
            crate::services::PlaybackService::new(self.config.clone())?
        ));

        tracing::debug!("Service manager initialization complete");
        Ok(())
    }

    /// Get library service
    pub fn library_service(&self) -> Option<Arc<crate::services::LibraryService>> {
        self.library_service.clone()
    }

    /// Get playback service
    pub fn playback_service(&self) -> Option<Arc<crate::services::PlaybackService>> {
        self.playback_service.clone()
    }

    /// Get settings service
    pub fn settings_service(&self) -> Option<Arc<crate::services::SettingsService>> {
        self.settings_service.clone()
    }

    /// Shutdown all services
    pub async fn shutdown(&mut self) {
        tracing::info!("Shutting down service manager");

        // Shutdown services in reverse order
        if let Some(_playback) = self.playback_service.take() {
            // TODO: Implement proper shutdown for playback service
            tracing::debug!("Shutting down playback service");
        }

        if let Some(_library) = self.library_service.take() {
            // TODO: Implement proper shutdown for library service
            tracing::debug!("Shutting down library service");
        }

        if let Some(_settings) = self.settings_service.take() {
            // TODO: Implement proper shutdown for settings service
            tracing::debug!("Shutting down settings service");
        }

        tracing::debug!("Service manager shutdown complete");
    }
}

impl Global for ServiceManager {}