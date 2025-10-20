//! XIM client management and communication

use crate::{XimError, XimResult, XimConnectionInfo, XimInputContext, XimPreeditInfo, XimStatusInfo};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// XIM client connection
#[derive(Clone)]
pub struct XimClient {
    /// Client ID
    pub client_id: u32,
    /// Client information
    pub connection_info: XimConnectionInfo,
    /// Input contexts
    pub input_contexts: HashMap<u32, XimInputContext>,
    /// Connection status
    pub connected: bool,
    /// Last activity time
    pub last_activity: std::time::SystemTime,
}

impl XimClient {
    /// Create a new client
    pub fn new(client_id: u32, connection_info: XimConnectionInfo) -> Self {
        Self {
            client_id,
            connection_info,
            input_contexts: HashMap::new(),
            connected: true,
            last_activity: std::time::SystemTime::now(),
        }
    }

    /// Add an input context
    pub fn add_input_context(&mut self, ic: XimInputContext) {
        self.input_contexts.insert(ic.ic_id, ic);
        self.last_activity = std::time::SystemTime::now();
    }

    /// Remove an input context
    pub fn remove_input_context(&mut self, ic_id: u32) -> Option<XimInputContext> {
        self.last_activity = std::time::SystemTime::now();
        self.input_contexts.remove(&ic_id)
    }

    /// Get an input context
    pub fn get_input_context(&self, ic_id: u32) -> Option<&XimInputContext> {
        self.input_contexts.get(&ic_id)
    }

    /// Get mutable input context
    pub fn get_input_context_mut(&mut self, ic_id: u32) -> Option<&mut XimInputContext> {
        self.last_activity = std::time::SystemTime::now();
        self.input_contexts.get_mut(&ic_id)
    }

    /// Update input context
    pub fn update_input_context(&mut self, ic_id: u32, updates: XimInputContextUpdate) -> XimResult<()> {
        if let Some(ic) = self.input_contexts.get_mut(&ic_id) {
            updates.apply(ic);
            self.last_activity = std::time::SystemTime::now();
            Ok(())
        } else {
            Err(XimError::ClientConnectionError(format!("Input context {} not found", ic_id)))
        }
    }

    /// Check if client is active
    pub fn is_active(&self, timeout: std::time::Duration) -> bool {
        self.connected && 
        self.last_activity.elapsed().unwrap_or(std::time::Duration::MAX) < timeout
    }

    /// Disconnect client
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.input_contexts.clear();
    }
}

/// Input context update
#[derive(Debug, Clone)]
pub struct XimInputContextUpdate {
    /// Buffer update
    pub buffer: Option<String>,
    /// Cursor position update
    pub cursor_position: Option<usize>,
    /// Preedit attributes update
    pub preedit_attributes: Option<HashMap<String, String>>,
    /// Status attributes update
    pub status_attributes: Option<HashMap<String, String>>,
}

impl XimInputContextUpdate {
    /// Create a new update
    pub fn new() -> Self {
        Self {
            buffer: None,
            cursor_position: None,
            preedit_attributes: None,
            status_attributes: None,
        }
    }

    /// Set buffer
    pub fn with_buffer(mut self, buffer: String) -> Self {
        self.buffer = Some(buffer);
        self
    }

    /// Set cursor position
    pub fn with_cursor_position(mut self, position: usize) -> Self {
        self.cursor_position = Some(position);
        self
    }

    /// Set preedit attributes
    pub fn with_preedit_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.preedit_attributes = Some(attributes);
        self
    }

    /// Set status attributes
    pub fn with_status_attributes(mut self, attributes: HashMap<String, String>) -> Self {
        self.status_attributes = Some(attributes);
        self
    }

    /// Apply updates to input context
    fn apply(&self, ic: &mut XimInputContext) {
        if let Some(buffer) = &self.buffer {
            ic.current_buffer = buffer.clone();
        }
        if let Some(position) = self.cursor_position {
            ic.cursor_position = position;
        }
        if let Some(attributes) = &self.preedit_attributes {
            ic.preedit_attributes = attributes.clone();
        }
        if let Some(attributes) = &self.status_attributes {
            ic.status_attributes = attributes.clone();
        }
    }
}

impl Default for XimInputContextUpdate {
    fn default() -> Self {
        Self::new()
    }
}

/// XIM client manager
#[derive(Clone)]
pub struct XimClientManager {
    /// Clients
    clients: Arc<RwLock<HashMap<u32, XimClient>>>,
    /// Next client ID
    next_client_id: Arc<RwLock<u32>>,
    /// Client timeout
    client_timeout: std::time::Duration,
}

impl XimClientManager {
    /// Create a new client manager
    pub fn new(client_timeout: std::time::Duration) -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(1)),
            client_timeout,
        }
    }

    /// Add a new client
    pub async fn add_client(&self, connection_info: XimConnectionInfo) -> XimResult<u32> {
        let mut next_id = self.next_client_id.write().await;
        let client_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let client = XimClient::new(client_id, connection_info);
        
        let mut clients = self.clients.write().await;
        clients.insert(client_id, client);
        
        Ok(client_id)
    }

    /// Remove a client
    pub async fn remove_client(&self, client_id: u32) -> Option<XimClient> {
        let mut clients = self.clients.write().await;
        clients.remove(&client_id)
    }

    /// Get a client
    pub async fn get_client(&self, client_id: u32) -> Option<XimClient> {
        let clients = self.clients.read().await;
        clients.get(&client_id).cloned()
    }

    /// Get mutable client
    pub async fn get_client_mut(&self, client_id: u32) -> Option<XimClient> {
        let mut clients = self.clients.write().await;
        clients.remove(&client_id)
    }

    /// Update client
    pub async fn update_client(&self, client_id: u32, updates: impl FnOnce(&mut XimClient)) -> XimResult<()> {
        let mut clients = self.clients.write().await;
        if let Some(client) = clients.get_mut(&client_id) {
            updates(client);
            Ok(())
        } else {
            Err(XimError::ClientConnectionError(format!("Client {} not found", client_id)))
        }
    }

    /// Add input context to client
    pub async fn add_input_context(&self, client_id: u32, ic: XimInputContext) -> XimResult<()> {
        self.update_client(client_id, |client| {
            client.add_input_context(ic);
        }).await
    }

    /// Remove input context from client
    pub async fn remove_input_context(&self, client_id: u32, ic_id: u32) -> XimResult<Option<XimInputContext>> {
        let mut clients = self.clients.write().await;
        if let Some(client) = clients.get_mut(&client_id) {
            Ok(client.remove_input_context(ic_id))
        } else {
            Err(XimError::ClientConnectionError(format!("Client {} not found", client_id)))
        }
    }

    /// Get input context
    pub async fn get_input_context(&self, client_id: u32, ic_id: u32) -> XimResult<Option<XimInputContext>> {
        let clients = self.clients.read().await;
        if let Some(client) = clients.get(&client_id) {
            Ok(client.get_input_context(ic_id).cloned())
        } else {
            Err(XimError::ClientConnectionError(format!("Client {} not found", client_id)))
        }
    }

    /// Update input context
    pub async fn update_input_context(&self, client_id: u32, ic_id: u32, updates: XimInputContextUpdate) -> XimResult<()> {
        self.update_client(client_id, |client| {
            let _ = client.update_input_context(ic_id, updates);
        }).await
    }

    /// Clean up inactive clients
    pub async fn cleanup_inactive_clients(&self) -> Vec<u32> {
        let mut clients = self.clients.write().await;
        let inactive_clients: Vec<u32> = clients
            .iter()
            .filter(|(_, client)| !client.is_active(self.client_timeout))
            .map(|(id, _)| *id)
            .collect();

        for client_id in &inactive_clients {
            clients.remove(client_id);
        }

        inactive_clients
    }

    /// Get all clients
    pub async fn get_all_clients(&self) -> Vec<XimClient> {
        let clients = self.clients.read().await;
        clients.values().cloned().collect::<Vec<_>>()
    }

    /// Get client count
    pub async fn get_client_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.len()
    }

    /// Check if client exists
    pub async fn client_exists(&self, client_id: u32) -> bool {
        let clients = self.clients.read().await;
        clients.contains_key(&client_id)
    }
}

impl Default for XimClientManager {
    fn default() -> Self {
        Self::new(std::time::Duration::from_secs(300)) // 5 minutes timeout
    }
}
