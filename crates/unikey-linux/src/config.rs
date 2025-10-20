//! XIM configuration and settings

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use unikey_input_methods::InputMethodType;

/// XIM server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XimConfig {
    /// Server name
    pub server_name: String,
    /// Display name
    pub display_name: Option<String>,
    /// Default input method
    pub default_input_method: InputMethodType,
    /// Available input methods
    pub available_input_methods: Vec<InputMethodType>,
    /// Hotkey configuration
    pub hotkeys: HotkeyConfig,
    /// UI configuration
    pub ui: UiConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Advanced settings
    pub advanced: AdvancedConfig,
}

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// Toggle input method hotkey
    pub toggle_input_method: Option<String>,
    /// Switch to Telex hotkey
    pub switch_to_telex: Option<String>,
    /// Switch to VNI hotkey
    pub switch_to_vni: Option<String>,
    /// Switch to VIQR hotkey
    pub switch_to_viqr: Option<String>,
    /// Toggle free marking hotkey
    pub toggle_free_marking: Option<String>,
    /// Show configuration hotkey
    pub show_config: Option<String>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Show status indicator
    pub show_status_indicator: bool,
    /// Status indicator position
    pub status_indicator_position: StatusPosition,
    /// Status indicator style
    pub status_indicator_style: StatusStyle,
    /// Show input method name
    pub show_input_method_name: bool,
    /// Show current buffer
    pub show_current_buffer: bool,
    /// Theme
    pub theme: String,
}

/// Status indicator position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Custom { x: i32, y: i32 },
}

/// Status indicator style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusStyle {
    Text,
    Icon,
    Both,
    Minimal,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file_path: Option<String>,
    /// Log to system journal
    pub log_to_journal: bool,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Advanced configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval: u64,
    /// Max clients
    pub max_clients: usize,
    /// Buffer size
    pub buffer_size: usize,
    /// Enable debug mode
    pub debug_mode: bool,
    /// Custom XIM attributes
    pub custom_attributes: HashMap<String, String>,
}

impl Default for XimConfig {
    fn default() -> Self {
        Self {
            server_name: "unikey-rust".to_string(),
            display_name: None,
            default_input_method: InputMethodType::Telex,
            available_input_methods: vec![
                InputMethodType::Telex,
                InputMethodType::VNI,
                InputMethodType::VIQR,
            ],
            hotkeys: HotkeyConfig::default(),
            ui: UiConfig::default(),
            logging: LoggingConfig::default(),
            advanced: AdvancedConfig::default(),
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_input_method: Some("Ctrl+Space".to_string()),
            switch_to_telex: Some("Ctrl+1".to_string()),
            switch_to_vni: Some("Ctrl+2".to_string()),
            switch_to_viqr: Some("Ctrl+3".to_string()),
            toggle_free_marking: Some("Ctrl+Shift+F".to_string()),
            show_config: Some("Ctrl+Shift+C".to_string()),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_status_indicator: true,
            status_indicator_position: StatusPosition::TopRight,
            status_indicator_style: StatusStyle::Both,
            show_input_method_name: true,
            show_current_buffer: false,
            theme: "default".to_string(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_to_file: false,
            log_file_path: None,
            log_to_journal: true,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            heartbeat_interval: 5,
            max_clients: 100,
            buffer_size: 4096,
            debug_mode: false,
            custom_attributes: HashMap::new(),
        }
    }
}

impl XimConfig {
    /// Create a new XIM configuration
    pub fn new(server_name: String) -> Self {
        Self {
            server_name,
            ..Default::default()
        }
    }

    /// Load configuration from file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: XimConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.server_name.is_empty() {
            return Err("Server name cannot be empty".to_string());
        }

        if self.available_input_methods.is_empty() {
            return Err("At least one input method must be available".to_string());
        }

        if !self.available_input_methods.contains(&self.default_input_method) {
            return Err("Default input method must be in available input methods".to_string());
        }

        if self.advanced.connection_timeout == 0 {
            return Err("Connection timeout must be greater than 0".to_string());
        }

        if self.advanced.heartbeat_interval == 0 {
            return Err("Heartbeat interval must be greater than 0".to_string());
        }

        if self.advanced.max_clients == 0 {
            return Err("Max clients must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Get input method by name
    pub fn get_input_method(&self, name: &str) -> Option<InputMethodType> {
        self.available_input_methods
            .iter()
            .find(|&method| method.to_string() == name)
            .copied()
    }

    /// Add input method
    pub fn add_input_method(&mut self, method: InputMethodType) {
        if !self.available_input_methods.contains(&method) {
            self.available_input_methods.push(method);
        }
    }

    /// Remove input method
    pub fn remove_input_method(&mut self, method: InputMethodType) {
        self.available_input_methods.retain(|&m| m != method);
        
        // If we removed the default input method, set a new default
        if self.default_input_method == method && !self.available_input_methods.is_empty() {
            self.default_input_method = self.available_input_methods[0];
        }
    }
}
