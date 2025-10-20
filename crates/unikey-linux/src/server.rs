//! XIM server implementation

use crate::{
    XimError, XimResult, XimConfig, XimMessage, XimMessageType, XimProtocolHandler,
    XimClientManager, XimConnectionInfo, XimInputContext, XimPreeditInfo, XimStatusInfo,
};
use unikey_core::UniKeyEngine;
use unikey_input_methods::InputMethodType;
use unikey_input_methods::prelude::*;
use unikey_macro::prelude::*;
use unikey_spell::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use std::collections::HashMap;

/// XIM server
pub struct XimServer {
    /// Server configuration
    config: XimConfig,
    /// Protocol handler
    protocol_handler: XimProtocolHandler,
    /// Client manager
    client_manager: XimClientManager,
    /// UniKey engine
    engine: Arc<RwLock<UniKeyEngine>>,
    /// Macro engine
    macro_engine: Arc<RwLock<MacroEngine>>,
    /// Spell checker
    spell_checker: Arc<RwLock<SpellChecker>>,
    /// Server status
    running: Arc<RwLock<bool>>,
    /// Input method registry
    input_methods: HashMap<InputMethodType, Box<dyn InputMethod + Send + Sync>>,
}

impl XimServer {
    /// Create a new XIM server
    pub async fn new(config: XimConfig) -> XimResult<Self> {
        // Validate configuration
        config.validate()
            .map_err(|e| XimError::ConfigurationError(e))?;

        // Initialize UniKey engine
        let engine = Arc::new(RwLock::new(UniKeyEngine::new()));

        // Initialize macro engine
        let macro_engine = Arc::new(RwLock::new(MacroEngine::new()));

        // Initialize spell checker
        let spell_checker = Arc::new(RwLock::new(SpellChecker::new()?));

        // Initialize input methods
        let mut input_methods: HashMap<InputMethodType, Box<dyn InputMethod + Send + Sync>> = HashMap::new();
        input_methods.insert(InputMethodType::Telex, Box::new(TelexMethod::new()));
        input_methods.insert(InputMethodType::VNI, Box::new(VniMethod::new()));
        input_methods.insert(InputMethodType::VIQR, Box::new(ViqrMethod::new()));

        let mut server = Self {
            config,
            protocol_handler: XimProtocolHandler::new(),
            client_manager: XimClientManager::new(Duration::from_secs(300)),
            engine,
            macro_engine,
            spell_checker,
            running: Arc::new(RwLock::new(false)),
            input_methods,
        };

        // Register message handlers
        server.register_handlers().await?;

        Ok(server)
    }

    /// Register XIM message handlers
    async fn register_handlers(&mut self) -> XimResult<()> {
        // For now, just register basic handlers
        // TODO: Implement proper handler registration with Arc<Self>
        Ok(())
    }

    /// Start the XIM server
    pub async fn run(&mut self) -> XimResult<()> {
        log::info!("Starting XIM server: {}", self.config.server_name);

        // Set running status
        {
            let mut running = self.running.write().await;
            *running = true;
        }

        // Start cleanup task
        let client_manager = self.client_manager.clone();
        let running = self.running.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Cleanup every minute
            while *running.read().await {
                interval.tick().await;
                let inactive_clients = client_manager.cleanup_inactive_clients().await;
                if !inactive_clients.is_empty() {
                    log::info!("Cleaned up {} inactive clients", inactive_clients.len());
                }
            }
        });

        // Main server loop
        self.main_loop().await?;

        Ok(())
    }

    /// Main server loop
    async fn main_loop(&self) -> XimResult<()> {
        log::info!("XIM server main loop started");

        // TODO: Implement actual XIM server loop
        // This would typically involve:
        // 1. Accepting client connections
        // 2. Processing XIM messages
        // 3. Handling input events
        // 4. Managing input contexts

        // For now, just keep the server running
        while *self.running.read().await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        log::info!("XIM server main loop ended");
        Ok(())
    }

    /// Stop the XIM server
    pub async fn stop(&self) -> XimResult<()> {
        log::info!("Stopping XIM server");

        // Set running status to false
        {
            let mut running = self.running.write().await;
            *running = false;
        }

        // Disconnect all clients
        let clients = self.client_manager.get_all_clients().await;
        for client in clients {
            log::info!("Disconnecting client: {}", client.client_id);
            // TODO: Send disconnect message to client
        }

        log::info!("XIM server stopped");
        Ok(())
    }

    /// Handle connect message
    fn handle_connect(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse connection info from message
            let connection_info = XimConnectionInfo {
                client_id: 0, // Will be assigned by client manager
                client_name: "Unknown".to_string(),
                client_version: 0,
                auth_data: Vec::new(),
                connection_time: std::time::SystemTime::now(),
            };

            // TODO: Add client to manager
            // TODO: Send connect reply

            Ok(XimMessage::new(XimMessageType::ConnectReply, Vec::new()))
        }
    }

    /// Handle disconnect message
    fn handle_disconnect(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse client ID from message
            // TODO: Remove client from manager
            // TODO: Send disconnect reply

            Ok(XimMessage::new(XimMessageType::ConnectReply, Vec::new()))
        }
    }

    /// Handle create input context message
    fn handle_create_ic(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse create IC parameters from message
            let ic = XimInputContext {
                ic_id: 0, // Will be assigned
                client_window: 0,
                focus_window: 0,
                input_method: "telex".to_string(),
                preedit_attributes: HashMap::new(),
                status_attributes: HashMap::new(),
                current_buffer: String::new(),
                cursor_position: 0,
            };

            // TODO: Add IC to client
            // TODO: Send create IC reply

            Ok(XimMessage::new(XimMessageType::CreateICReply, Vec::new()))
        }
    }

    /// Handle destroy input context message
    fn handle_destroy_ic(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse IC ID from message
            // TODO: Remove IC from client
            // TODO: Send destroy IC reply

            Ok(XimMessage::new(XimMessageType::DestroyICReply, Vec::new()))
        }
    }

    /// Handle set IC values message
    fn handle_set_ic_values(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse IC values from message
            // TODO: Update IC values
            // TODO: Send set IC values reply

            Ok(XimMessage::new(XimMessageType::SetICValuesReply, Vec::new()))
        }
    }

    /// Handle get IC values message
    fn handle_get_ic_values(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse IC ID from message
            // TODO: Get IC values
            // TODO: Send get IC values reply

            Ok(XimMessage::new(XimMessageType::GetICValuesReply, Vec::new()))
        }
    }

    /// Handle preedit start message
    fn handle_preedit_start(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse preedit start parameters from message
            // TODO: Start preedit for IC
            // TODO: Send preedit start reply

            Ok(XimMessage::new(XimMessageType::PreeditStartReply, Vec::new()))
        }
    }

    /// Handle preedit draw message
    fn handle_preedit_draw(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse preedit draw parameters from message
            // TODO: Update preedit for IC
            // TODO: Send preedit draw reply

            Ok(XimMessage::new(XimMessageType::PreeditDrawReply, Vec::new()))
        }
    }

    /// Handle preedit done message
    fn handle_preedit_done(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse preedit done parameters from message
            // TODO: End preedit for IC
            // TODO: Send preedit done reply

            Ok(XimMessage::new(XimMessageType::PreeditDoneReply, Vec::new()))
        }
    }

    /// Handle commit message
    fn handle_commit(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse commit parameters from message
            // TODO: Commit text for IC
            // TODO: Send commit reply

            Ok(XimMessage::new(XimMessageType::CommitReply, Vec::new()))
        }
    }

    /// Handle forward event message
    fn handle_forward_event(server: Arc<Self>) -> impl Fn(&XimMessage) -> XimResult<XimMessage> + Send + Sync {
        move |message| {
            // TODO: Parse event from message
            // TODO: Process event with UniKey engine
            // TODO: Send forward event reply

            Ok(XimMessage::new(XimMessageType::ForwardEventReply, Vec::new()))
        }
    }

    /// Get server configuration
    pub fn config(&self) -> &XimConfig {
        &self.config
    }

    /// Get client count
    pub async fn get_client_count(&self) -> usize {
        self.client_manager.get_client_count().await
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Clone for XimServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            protocol_handler: XimProtocolHandler::new(),
            client_manager: self.client_manager.clone(),
            engine: self.engine.clone(),
            macro_engine: self.macro_engine.clone(),
            spell_checker: self.spell_checker.clone(),
            running: self.running.clone(),
            input_methods: HashMap::new(), // Will be re-initialized
        }
    }
}
