//! Client manager for multi-server support
//!
//! This module provides a ClientManager that can handle multiple Jellyfin server
//! connections simultaneously, with server switching, state isolation, and
//! connection pooling for efficient resource usage.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};

use super::client::{ConnectionEvent, ConnectionEventListener, JellyfinClient, ReconnectConfig};
use super::error::JellyfinError;
use crate::models::api::PublicServerInfo;
use crate::models::server::ServerConfig;

/// Server connection pool entry
#[derive(Debug)]
struct PooledConnection {
    /// The Jellyfin client instance
    client: JellyfinClient,
    /// Whether this connection is currently active
    is_active: bool,
    /// Reference count for connection sharing
    ref_count: u32,
    /// Server configuration
    server_config: ServerConfig,
}

/// Multi-server client manager
///
/// This manager handles multiple Jellyfin server connections, providing
/// server switching, state isolation, and efficient connection pooling.
pub struct ClientManager {
    /// Pool of server connections indexed by server ID
    connection_pool: Arc<RwLock<HashMap<String, PooledConnection>>>,
    /// Currently active server ID
    active_server_id: Arc<RwLock<Option<String>>>,
    /// Default reconnection configuration for new clients
    default_reconnect_config: ReconnectConfig,
    /// Event listeners for manager-level events
    event_listeners: Arc<Mutex<Vec<Arc<dyn ClientManagerEventListener>>>>,
}

/// Client manager events
#[derive(Debug, Clone)]
pub enum ClientManagerEvent {
    /// A server was added to the pool
    ServerAdded {
        server_id: String,
        server_config: ServerConfig,
    },
    /// A server was removed from the pool
    ServerRemoved {
        server_id: String,
    },
    /// Active server was switched
    ActiveServerChanged {
        old_server_id: Option<String>,
        new_server_id: Option<String>,
    },
    /// Connection established to a server
    ServerConnected {
        server_id: String,
        server_info: PublicServerInfo,
    },
    /// Connection lost to a server
    ServerDisconnected {
        server_id: String,
        reason: String,
    },
    /// Error occurred with a server
    ServerError {
        server_id: String,
        error: String,
    },
}

/// Event listener trait for client manager events
pub trait ClientManagerEventListener: Send + Sync {
    /// Called when a client manager event occurs
    fn on_manager_event(&self, event: ClientManagerEvent);
}

/// Connection event forwarder that bridges client events to manager events
struct ConnectionEventForwarder {
    server_id: String,
    manager_listeners: Arc<Mutex<Vec<Arc<dyn ClientManagerEventListener>>>>,
}

impl ConnectionEventListener for ConnectionEventForwarder {
    fn on_connection_event(&self, event: ConnectionEvent) {
        let server_id = self.server_id.clone();
        let listeners = Arc::clone(&self.manager_listeners);

        // Convert client events to manager events
        let manager_event = match event {
            ConnectionEvent::StateChanged { new_state, .. } => {
                match new_state {
                    super::client::ConnectionState::Connected => None, // Handled by ServerInfoUpdated
                    super::client::ConnectionState::Failed(reason) => {
                        Some(ClientManagerEvent::ServerDisconnected { server_id, reason })
                    }
                    _ => None,
                }
            }
            ConnectionEvent::ServerInfoUpdated { server_info, .. } => {
                Some(ClientManagerEvent::ServerConnected { server_id, server_info })
            }
            ConnectionEvent::Error { error, .. } => {
                Some(ClientManagerEvent::ServerError { server_id, error })
            }
            _ => None,
        };

        if let Some(event) = manager_event {
            // Spawn a task to notify listeners asynchronously
            tokio::spawn(async move {
                let listeners_guard = listeners.lock().await;
                for listener in listeners_guard.iter() {
                    listener.on_manager_event(event.clone());
                }
            });
        }
    }
}

impl ClientManager {
    /// Create a new client manager
    pub fn new() -> Self {
        Self {
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            active_server_id: Arc::new(RwLock::new(None)),
            default_reconnect_config: ReconnectConfig::default(),
            event_listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create a new client manager with custom reconnection configuration
    pub fn with_reconnect_config(reconnect_config: ReconnectConfig) -> Self {
        Self {
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            active_server_id: Arc::new(RwLock::new(None)),
            default_reconnect_config: reconnect_config,
            event_listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add an event listener
    pub async fn add_event_listener(&self, listener: Arc<dyn ClientManagerEventListener>) {
        self.event_listeners.lock().await.push(listener);
    }

    /// Remove all event listeners
    pub async fn clear_event_listeners(&self) {
        self.event_listeners.lock().await.clear();
    }

    /// Notify all event listeners
    async fn notify_event_listeners(&self, event: ClientManagerEvent) {
        let listeners = self.event_listeners.lock().await;
        for listener in listeners.iter() {
            listener.on_manager_event(event.clone());
        }
    }

    /// Add a server to the connection pool
    ///
    /// Creates a new client instance for the server but doesn't connect immediately.
    /// Use `connect_server()` to establish the connection.
    pub async fn add_server(&self, server_config: ServerConfig) -> Result<()> {
        let server_id = server_config.id.clone();
        info!("Adding server to pool: {} ({})", server_config.name, server_id);

        // Check if server already exists
        {
            let pool = self.connection_pool.read().await;
            if pool.contains_key(&server_id) {
                return Err(JellyfinError::configuration(
                    format!("Server with ID '{}' already exists", server_id)
                ).into());
            }
        }

        // Create a new client with the default reconnect config
        let client = JellyfinClient::with_reconnect_config(self.default_reconnect_config.clone());

        // Set up event forwarding
        let forwarder = Arc::new(ConnectionEventForwarder {
            server_id: server_id.clone(),
            manager_listeners: Arc::clone(&self.event_listeners),
        });
        client.add_event_listener(forwarder).await;

        // Add to pool
        let pooled_connection = PooledConnection {
            client,
            is_active: false,
            ref_count: 0,
            server_config: server_config.clone(),
        };

        {
            let mut pool = self.connection_pool.write().await;
            pool.insert(server_id.clone(), pooled_connection);
        }

        // Notify listeners
        self.notify_event_listeners(ClientManagerEvent::ServerAdded {
            server_id,
            server_config,
        }).await;

        Ok(())
    }

    /// Remove a server from the connection pool
    ///
    /// Disconnects the server if connected and removes it from the pool.
    /// If this was the active server, no server will be active after removal.
    pub async fn remove_server(&self, server_id: &str) -> Result<()> {
        info!("Removing server from pool: {}", server_id);

        // Check if this is the active server
        let was_active = {
            let active_id = self.active_server_id.read().await;
            active_id.as_ref() == Some(&server_id.to_string())
        };

        // Remove from pool
        let removed = {
            let mut pool = self.connection_pool.write().await;
            pool.remove(server_id)
        };

        match removed {
            Some(mut pooled_conn) => {
                // Disconnect the client
                pooled_conn.client.disconnect().await;

                // Clear active server if this was it
                if was_active {
                    *self.active_server_id.write().await = None;
                    self.notify_event_listeners(ClientManagerEvent::ActiveServerChanged {
                        old_server_id: Some(server_id.to_string()),
                        new_server_id: None,
                    }).await;
                }

                // Notify listeners
                self.notify_event_listeners(ClientManagerEvent::ServerRemoved {
                    server_id: server_id.to_string(),
                }).await;

                Ok(())
            }
            None => Err(JellyfinError::configuration(
                format!("Server with ID '{}' not found", server_id)
            ).into()),
        }
    }

    /// Connect to a server in the pool
    ///
    /// Establishes a connection to the specified server. The server must
    /// already be added to the pool using `add_server()`.
    pub async fn connect_server(&self, server_id: &str) -> Result<PublicServerInfo> {
        info!("Connecting to server: {}", server_id);

        let server_info = {
            let mut pool = self.connection_pool.write().await;
            match pool.get_mut(server_id) {
                Some(pooled_conn) => {
                    let server_url = &pooled_conn.server_config.url;
                    pooled_conn.client.connect(server_url).await?
                }
                None => {
                    return Err(JellyfinError::configuration(
                        format!("Server with ID '{}' not found in pool", server_id)
                    ).into());
                }
            }
        };

        debug!("Successfully connected to server: {}", server_id);
        Ok(server_info)
    }

    /// Disconnect from a server in the pool
    ///
    /// Disconnects from the specified server but keeps it in the pool.
    /// If this was the active server, no server will be active after disconnection.
    pub async fn disconnect_server(&self, server_id: &str) -> Result<()> {
        info!("Disconnecting from server: {}", server_id);

        // Check if this is the active server
        let was_active = {
            let active_id = self.active_server_id.read().await;
            active_id.as_ref() == Some(&server_id.to_string())
        };

        // Disconnect the client
        {
            let mut pool = self.connection_pool.write().await;
            match pool.get_mut(server_id) {
                Some(pooled_conn) => {
                    pooled_conn.client.disconnect().await;
                    pooled_conn.is_active = false;
                }
                None => {
                    return Err(JellyfinError::configuration(
                        format!("Server with ID '{}' not found in pool", server_id)
                    ).into());
                }
            }
        }

        // Clear active server if this was it
        if was_active {
            *self.active_server_id.write().await = None;
            self.notify_event_listeners(ClientManagerEvent::ActiveServerChanged {
                old_server_id: Some(server_id.to_string()),
                new_server_id: None,
            }).await;
        }

        Ok(())
    }

    /// Set the active server
    ///
    /// Switches the active server to the specified server ID. The server
    /// must be in the pool and connected.
    pub async fn set_active_server(&self, server_id: &str) -> Result<()> {
        info!("Setting active server to: {}", server_id);

        // Verify the server exists and is connected
        {
            let pool = self.connection_pool.read().await;
            match pool.get(server_id) {
                Some(pooled_conn) => {
                    if !pooled_conn.client.is_connected() {
                        return Err(JellyfinError::configuration(
                            format!("Server '{}' is not connected", server_id)
                        ).into());
                    }
                }
                None => {
                    return Err(JellyfinError::configuration(
                        format!("Server with ID '{}' not found in pool", server_id)
                    ).into());
                }
            }
        }

        // Update active server
        let old_server_id = {
            let mut active_id = self.active_server_id.write().await;
            let old_id = active_id.clone();
            *active_id = Some(server_id.to_string());
            old_id
        };

        // Update active status in pool
        {
            let mut pool = self.connection_pool.write().await;

            // Deactivate old server
            if let Some(old_id) = &old_server_id {
                if let Some(old_conn) = pool.get_mut(old_id) {
                    old_conn.is_active = false;
                }
            }

            // Activate new server
            if let Some(new_conn) = pool.get_mut(server_id) {
                new_conn.is_active = true;
            }
        }

        // Notify listeners
        self.notify_event_listeners(ClientManagerEvent::ActiveServerChanged {
            old_server_id,
            new_server_id: Some(server_id.to_string()),
        }).await;

        Ok(())
    }

    /// Get the currently active server ID
    pub async fn get_active_server_id(&self) -> Option<String> {
        self.active_server_id.read().await.clone()
    }

    /// Get a client for the active server
    ///
    /// Returns a clone of the client for the currently active server.
    /// The client can be used for API operations.
    pub async fn get_active_client(&self) -> Result<JellyfinClient> {
        let active_id = self.active_server_id.read().await.clone();

        match active_id {
            Some(server_id) => self.get_client(&server_id).await,
            None => Err(JellyfinError::configuration("No active server set").into()),
        }
    }

    /// Get a client for a specific server
    ///
    /// Returns a clone of the client for the specified server.
    /// The server must be in the pool.
    pub async fn get_client(&self, server_id: &str) -> Result<JellyfinClient> {
        let pool = self.connection_pool.read().await;

        match pool.get(server_id) {
            Some(pooled_conn) => {
                // Increment reference count
                // Note: In a real implementation, you might want to track this more carefully
                Ok(pooled_conn.client.clone())
            }
            None => Err(JellyfinError::configuration(
                format!("Server with ID '{}' not found in pool", server_id)
            ).into()),
        }
    }

    /// List all servers in the pool
    ///
    /// Returns a list of server configurations for all servers in the pool.
    pub async fn list_servers(&self) -> Vec<ServerConfig> {
        let pool = self.connection_pool.read().await;
        pool.values().map(|conn| conn.server_config.clone()).collect()
    }

    /// Get server configuration by ID
    pub async fn get_server_config(&self, server_id: &str) -> Option<ServerConfig> {
        let pool = self.connection_pool.read().await;
        pool.get(server_id).map(|conn| conn.server_config.clone())
    }

    /// Check if a server is connected
    pub async fn is_server_connected(&self, server_id: &str) -> bool {
        let pool = self.connection_pool.read().await;
        pool.get(server_id)
            .map(|conn| conn.client.is_connected())
            .unwrap_or(false)
    }

    /// Check if a server is active
    pub async fn is_server_active(&self, server_id: &str) -> bool {
        let pool = self.connection_pool.read().await;
        pool.get(server_id)
            .map(|conn| conn.is_active)
            .unwrap_or(false)
    }

    /// Get connection information for a server
    pub async fn get_server_connection_info(&self, server_id: &str) -> Option<super::client::ConnectionInfo> {
        let pool = self.connection_pool.read().await;
        match pool.get(server_id) {
            Some(conn) => Some(conn.client.get_connection_info().await),
            None => None,
        }
    }

    /// Check all server connections and attempt reconnection if needed
    ///
    /// This method iterates through all servers in the pool and checks their
    /// connection status, attempting reconnection for failed connections.
    pub async fn check_all_connections(&self) -> Result<()> {
        info!("Checking all server connections");

        let server_ids: Vec<String> = {
            let pool = self.connection_pool.read().await;
            pool.keys().cloned().collect()
        };

        for server_id in server_ids {
            if let Ok(mut client) = self.get_client(&server_id).await {
                if let Err(e) = client.check_connection().await {
                    warn!("Connection check failed for server {}: {}", server_id, e);
                }
            }
        }

        Ok(())
    }

    /// Disconnect from all servers
    ///
    /// Disconnects from all servers in the pool but keeps them in the pool.
    /// Clears the active server.
    pub async fn disconnect_all(&self) -> Result<()> {
        info!("Disconnecting from all servers");

        let server_ids: Vec<String> = {
            let pool = self.connection_pool.read().await;
            pool.keys().cloned().collect()
        };

        for server_id in server_ids {
            if let Err(e) = self.disconnect_server(&server_id).await {
                warn!("Failed to disconnect from server {}: {}", server_id, e);
            }
        }

        // Clear active server
        *self.active_server_id.write().await = None;

        Ok(())
    }

    /// Get pool statistics
    pub async fn get_pool_stats(&self) -> PoolStats {
        let pool = self.connection_pool.read().await;

        let total_servers = pool.len();
        let connected_servers = pool.values().filter(|conn| conn.client.is_connected()).count();
        let active_servers = pool.values().filter(|conn| conn.is_active).count();

        PoolStats {
            total_servers,
            connected_servers,
            active_servers,
        }
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Total number of servers in the pool
    pub total_servers: usize,
    /// Number of connected servers
    pub connected_servers: usize,
    /// Number of active servers (should be 0 or 1)
    pub active_servers: usize,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ClientManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientManager")
            .field("default_reconnect_config", &self.default_reconnect_config)
            .finish_non_exhaustive()
    }
}

/// Convenience methods for common multi-server operations
impl ClientManager {
    /// Add and connect to a server in one operation
    ///
    /// This is a convenience method that adds a server to the pool and
    /// immediately attempts to connect to it.
    pub async fn add_and_connect_server(&self, server_config: ServerConfig) -> Result<PublicServerInfo> {
        let server_id = server_config.id.clone();

        // Add the server to the pool
        self.add_server(server_config).await?;

        // Connect to the server
        match self.connect_server(&server_id).await {
            Ok(server_info) => Ok(server_info),
            Err(e) => {
                // If connection failed, remove the server from the pool
                if let Err(remove_err) = self.remove_server(&server_id).await {
                    warn!("Failed to remove server after connection failure: {}", remove_err);
                }
                Err(e)
            }
        }
    }

    /// Switch to a different server
    ///
    /// This method switches the active server to a different server in the pool.
    /// If the target server is not connected, it will attempt to connect first.
    pub async fn switch_server(&self, server_id: &str) -> Result<()> {
        info!("Switching to server: {}", server_id);

        // Check if the server is already connected
        if !self.is_server_connected(server_id).await {
            info!("Server {} not connected, attempting connection", server_id);
            self.connect_server(server_id).await?;
        }

        // Set as active server
        self.set_active_server(server_id).await?;

        Ok(())
    }

    /// Get or create a client for a server
    ///
    /// This method returns a client for the specified server, connecting
    /// to the server if not already connected.
    pub async fn get_or_connect_client(&self, server_id: &str) -> Result<JellyfinClient> {
        // Check if server exists in pool
        if self.get_server_config(server_id).await.is_none() {
            return Err(JellyfinError::configuration(
                format!("Server with ID '{}' not found in pool", server_id)
            ).into());
        }

        // Connect if not already connected
        if !self.is_server_connected(server_id).await {
            self.connect_server(server_id).await?;
        }

        // Return the client
        self.get_client(server_id).await
    }

    /// Find servers by name (case-insensitive partial match)
    pub async fn find_servers_by_name(&self, name_pattern: &str) -> Vec<ServerConfig> {
        let pool = self.connection_pool.read().await;
        let pattern = name_pattern.to_lowercase();

        pool.values()
            .filter(|conn| conn.server_config.name.to_lowercase().contains(&pattern))
            .map(|conn| conn.server_config.clone())
            .collect()
    }

    /// Get all connected servers
    pub async fn get_connected_servers(&self) -> Vec<ServerConfig> {
        let pool = self.connection_pool.read().await;

        pool.values()
            .filter(|conn| conn.client.is_connected())
            .map(|conn| conn.server_config.clone())
            .collect()
    }

    /// Get all disconnected servers
    pub async fn get_disconnected_servers(&self) -> Vec<ServerConfig> {
        let pool = self.connection_pool.read().await;

        pool.values()
            .filter(|conn| !conn.client.is_connected())
            .map(|conn| conn.server_config.clone())
            .collect()
    }
}