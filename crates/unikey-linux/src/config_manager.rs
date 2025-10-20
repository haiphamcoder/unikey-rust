//! Configuration management for Linux XIM

use crate::{XimError, XimResult, XimConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use unikey_input_methods::InputMethodType;

/// Configuration file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub xim: XimConfig,
    pub hotkeys: HotkeyConfig,
    pub ui: UIConfig,
    pub advanced: AdvancedConfig,
}

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_input_method: String,
    pub switch_input_method: String,
    pub show_configuration: String,
    pub show_about: String,
    pub quit: String,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub show_status_indicator: bool,
    pub status_indicator_position: String,
    pub status_indicator_style: String,
    pub theme: String,
    pub language: String,
}

/// Advanced configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    pub debug_mode: bool,
    pub log_level: String,
    pub log_file: Option<String>,
    pub connection_timeout: u64,
    pub heartbeat_interval: u64,
    pub max_clients: u32,
}

/// Configuration manager
pub struct ConfigManager {
    config: Arc<RwLock<ConfigFile>>,
    config_path: PathBuf,
    watchers: Vec<Box<dyn ConfigWatcher>>,
}

/// Configuration watcher trait
pub trait ConfigWatcher: Send + Sync {
    fn on_config_changed(&mut self, config: &ConfigFile) -> XimResult<()>;
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new(config_path: Option<PathBuf>) -> XimResult<Self> {
        let config_path = config_path.unwrap_or_else(|| {
            let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("unikey-rust");
            path.push("config.json");
            path
        });
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Load or create default config
        let config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            Self::create_default_config()
        };
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            watchers: Vec::new(),
        })
    }
    
    /// Load configuration from file
    pub fn load_config(path: &PathBuf) -> XimResult<ConfigFile> {
        let content = fs::read_to_string(path)?;
        let config: ConfigFile = serde_json::from_str(&content)
            .map_err(|e| XimError::SerializationError(e))?;
        Ok(config)
    }
    
    /// Create default configuration
    pub fn create_default_config() -> ConfigFile {
        ConfigFile {
            xim: XimConfig::new("unikey-rust".to_string()),
            hotkeys: HotkeyConfig {
                toggle_input_method: "Ctrl+Space".to_string(),
                switch_input_method: "Ctrl+Shift+Space".to_string(),
                show_configuration: "Ctrl+Alt+C".to_string(),
                show_about: "Ctrl+Alt+A".to_string(),
                quit: "Ctrl+Alt+Q".to_string(),
            },
            ui: UIConfig {
                show_status_indicator: true,
                status_indicator_position: "top-right".to_string(),
                status_indicator_style: "modern".to_string(),
                theme: "auto".to_string(),
                language: "en".to_string(),
            },
            advanced: AdvancedConfig {
                debug_mode: false,
                log_level: "info".to_string(),
                log_file: None,
                connection_timeout: 30,
                heartbeat_interval: 5,
                max_clients: 100,
            },
        }
    }
    
    /// Save configuration to file
    pub fn save_config(&self) -> XimResult<()> {
        let config = self.config.read().unwrap();
        let content = serde_json::to_string_pretty(&*config)
            .map_err(|e| XimError::SerializationError(e))?;
        fs::write(&self.config_path, content)?;
        log::info!("Configuration saved to {:?}", self.config_path);
        Ok(())
    }
    
    /// Get configuration
    pub fn get_config(&self) -> ConfigFile {
        self.config.read().unwrap().clone()
    }
    
    /// Update configuration
    pub fn update_config<F>(&mut self, updater: F) -> XimResult<()>
    where
        F: FnOnce(&mut ConfigFile),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut config);
        
        // Notify watchers
        for watcher in &mut self.watchers {
            if let Err(e) = watcher.on_config_changed(&*config) {
                log::error!("Config watcher error: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Add configuration watcher
    pub fn add_watcher(&mut self, watcher: Box<dyn ConfigWatcher>) {
        self.watchers.push(watcher);
    }
    
    /// Get XIM configuration
    pub fn get_xim_config(&self) -> XimConfig {
        self.config.read().unwrap().xim.clone()
    }
    
    /// Update XIM configuration
    pub fn update_xim_config<F>(&mut self, updater: F) -> XimResult<()>
    where
        F: FnOnce(&mut XimConfig),
    {
        self.update_config(|config| {
            updater(&mut config.xim);
        })
    }
    
    /// Get hotkey configuration
    pub fn get_hotkeys(&self) -> HotkeyConfig {
        self.config.read().unwrap().hotkeys.clone()
    }
    
    /// Update hotkey configuration
    pub fn update_hotkeys<F>(&mut self, updater: F) -> XimResult<()>
    where
        F: FnOnce(&mut HotkeyConfig),
    {
        self.update_config(|config| {
            updater(&mut config.hotkeys);
        })
    }
    
    /// Get UI configuration
    pub fn get_ui_config(&self) -> UIConfig {
        self.config.read().unwrap().ui.clone()
    }
    
    /// Update UI configuration
    pub fn update_ui_config<F>(&mut self, updater: F) -> XimResult<()>
    where
        F: FnOnce(&mut UIConfig),
    {
        self.update_config(|config| {
            updater(&mut config.ui);
        })
    }
    
    /// Get advanced configuration
    pub fn get_advanced_config(&self) -> AdvancedConfig {
        self.config.read().unwrap().advanced.clone()
    }
    
    /// Update advanced configuration
    pub fn update_advanced_config<F>(&mut self, updater: F) -> XimResult<()>
    where
        F: FnOnce(&mut AdvancedConfig),
    {
        self.update_config(|config| {
            updater(&mut config.advanced);
        })
    }
    
    /// Reset to default configuration
    pub fn reset_to_default(&mut self) -> XimResult<()> {
        let default_config = Self::create_default_config();
        *self.config.write().unwrap() = default_config;
        
        // Notify watchers
        let config = self.config.read().unwrap();
        for watcher in &mut self.watchers {
            if let Err(e) = watcher.on_config_changed(&*config) {
                log::error!("Config watcher error: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Get configuration file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
    
    /// Check if configuration file exists
    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }
    
    /// Reload configuration from file
    pub fn reload(&mut self) -> XimResult<()> {
        if self.config_path.exists() {
            let new_config = Self::load_config(&self.config_path)?;
            *self.config.write().unwrap() = new_config;
            
            // Notify watchers
            let config = self.config.read().unwrap();
            for watcher in &mut self.watchers {
                if let Err(e) = watcher.on_config_changed(&*config) {
                    log::error!("Config watcher error: {}", e);
                }
            }
            
            log::info!("Configuration reloaded from file");
        }
        Ok(())
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_input_method: "Ctrl+Space".to_string(),
            switch_input_method: "Ctrl+Shift+Space".to_string(),
            show_configuration: "Ctrl+Alt+C".to_string(),
            show_about: "Ctrl+Alt+A".to_string(),
            quit: "Ctrl+Alt+Q".to_string(),
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            show_status_indicator: true,
            status_indicator_position: "top-right".to_string(),
            status_indicator_style: "modern".to_string(),
            theme: "auto".to_string(),
            language: "en".to_string(),
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            log_level: "info".to_string(),
            log_file: None,
            connection_timeout: 30,
            heartbeat_interval: 5,
            max_clients: 100,
        }
    }
}
