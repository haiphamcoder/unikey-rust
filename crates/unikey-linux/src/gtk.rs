//! GTK3 Input Method Module for Linux
//! 
//! This module provides GTK3 integration for UniKey Rust, including:
//! - GTK3 input method module implementation
//! - Input context management
//! - Preedit and commit handling
//! - GTK3 application integration
//! - Input method switching and configuration

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use gtk::prelude::*;
use gtk::{Application, Entry};

use crate::error::{XimError, XimResult};
use unikey_core::prelude::*;
use unikey_input_methods::{InputMethod as InputMethodTrait, TelexMethod};
use log::info;

/// Input result for GTK3 input method
#[derive(Debug, Clone)]
pub enum InputResult {
    /// Commit text
    Commit(String),
    /// Preedit text with cursor position
    Preedit(String, i32),
    /// Delete surrounding text
    DeleteSurrounding(u32, u32),
    /// No action
    None,
}

/// GTK3 Input Method Module
pub struct GtkInputMethodModule {
    /// Application instance
    app: Option<Application>,
    /// Input method engine
    engine: Option<Box<dyn InputMethodTrait + Send + Sync>>,
    /// Input context
    input_context: Option<GtkInputContext>,
    /// Configuration
    config: GtkInputMethodConfig,
    /// Running state
    running: Arc<std::sync::atomic::AtomicBool>,
}

/// GTK3 Input Context
pub struct GtkInputContext {
    /// Entry widget
    entry: Entry,
    /// Preedit text
    preedit_text: String,
    /// Cursor position
    cursor_position: i32,
    /// Input method state
    state: GtkInputMethodState,
    /// Key event buffer
    key_buffer: Vec<KeyEvent>,
}

/// GTK3 Input Method State
#[derive(Debug, Clone, PartialEq)]
pub enum GtkInputMethodState {
    /// Inactive state
    Inactive,
    /// Active state
    Active,
    /// Preedit state
    Preedit,
    /// Committed state
    Committed,
}

/// GTK3 Input Method Configuration
#[derive(Debug, Clone)]
pub struct GtkInputMethodConfig {
    /// Input method type
    pub input_method_type: String,
    /// Enable preedit
    pub preedit_enabled: bool,
    /// Enable commit
    pub commit_enabled: bool,
    /// Enable delete surrounding
    pub delete_surrounding_enabled: bool,
    /// Enable cursor position
    pub cursor_position_enabled: bool,
    /// Theme name
    pub theme_name: String,
    /// Font size
    pub font_size: i32,
    /// Show status indicator
    pub show_status_indicator: bool,
}

impl Default for GtkInputMethodConfig {
    fn default() -> Self {
        Self {
            input_method_type: "telex".to_string(),
            preedit_enabled: true,
            commit_enabled: true,
            delete_surrounding_enabled: true,
            cursor_position_enabled: true,
            theme_name: "Adwaita".to_string(),
            font_size: 12,
            show_status_indicator: true,
        }
    }
}

/// GTK3 Input Method Event
#[derive(Debug, Clone)]
pub enum GtkInputMethodEvent {
    /// Key press event
    KeyPress {
        key: u32,
        state: u32,
        time: u32,
    },
    /// Key release event
    KeyRelease {
        key: u32,
        state: u32,
        time: u32,
    },
    /// Text input event
    TextInput {
        text: String,
    },
    /// Commit event
    Commit {
        text: String,
    },
    /// Preedit event
    Preedit {
        text: String,
        cursor: i32,
    },
    /// Delete surrounding text event
    DeleteSurrounding {
        before_length: u32,
        after_length: u32,
    },
    /// Cursor position event
    CursorPosition {
        x: f64,
        y: f64,
    },
    /// Focus in event
    FocusIn,
    /// Focus out event
    FocusOut,
    /// Input method changed event
    InputMethodChanged {
        method: String,
    },
}

/// GTK3 Input Method Event Handler
pub trait GtkInputMethodEventHandler: Send + Sync {
    /// Handle input method event
    fn handle_event(&mut self, event: GtkInputMethodEvent) -> XimResult<()>;
    
    /// Handle preedit text
    fn handle_preedit(&mut self, text: &str, cursor: i32) -> XimResult<()>;
    
    /// Handle commit text
    fn handle_commit(&mut self, text: &str) -> XimResult<()>;
    
    /// Handle input method change
    fn handle_input_method_change(&mut self, method: &str) -> XimResult<()>;
}

/// GTK3 Input Method Manager
pub struct GtkInputMethodManager {
    /// Input method module
    module: Arc<RwLock<GtkInputMethodModule>>,
    /// Event handlers
    event_handlers: Vec<Box<dyn GtkInputMethodEventHandler>>,
    /// Input contexts
    input_contexts: HashMap<String, GtkInputContext>,
    /// Configuration
    config: GtkInputMethodConfig,
}

impl GtkInputMethodModule {
    /// Create a new GTK3 input method module
    pub fn new() -> Self {
        Self {
            app: None,
            engine: None,
            input_context: None,
            config: GtkInputMethodConfig::default(),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Initialize GTK3 application
    pub fn initialize(&mut self) -> XimResult<()> {
        info!("Initializing GTK3 input method module...");
        
        // Initialize GTK
        if !gtk::is_initialized() {
            gtk::init().map_err(|e| XimError::GtkError(format!("Failed to initialize GTK: {}", e)))?;
        }
        
        // Create application
        let app = Application::new(
            Some("com.unikey.gtk_input_method"),
            gio::ApplicationFlags::empty(),
        );
        
        self.app = Some(app);
        info!("GTK3 input method module initialized successfully");
        
        Ok(())
    }
    
    /// Set input method engine
    pub fn set_engine(&mut self, engine: Box<dyn InputMethodTrait + Send + Sync>) {
        self.engine = Some(engine);
        info!("GTK3 input method engine set");
    }
    
    /// Create input context
    pub fn create_input_context(&mut self, entry: Entry) -> XimResult<GtkInputContext> {
        let context = GtkInputContext {
            entry,
            preedit_text: String::new(),
            cursor_position: 0,
            state: GtkInputMethodState::Inactive,
            key_buffer: Vec::new(),
        };
        
        self.input_context = Some(context.clone());
        info!("GTK3 input context created");
        
        Ok(context)
    }
    
    /// Process key event
    pub fn process_key_event(&mut self, key: u32, state: u32, time: u32) -> XimResult<()> {
        // Convert GTK key to KeyEvent first
        let key_event = self.gtk_key_to_key_event(key, state, time)?;
        
        if let Some(ref mut engine) = self.engine {
            // Process with engine
            let result = engine.process_key(key_event).map_err(|e| XimError::InputMethodError(e))?;
            
            // Convert Vec<u8> to InputResult for demo
            let input_result = InputResult::Commit(String::from_utf8_lossy(&result).to_string());
            self.handle_engine_result(input_result)?;
        }
        
        Ok(())
    }
    
    /// Handle engine result
    fn handle_engine_result(&mut self, result: InputResult) -> XimResult<()> {
        match result {
            InputResult::Commit(text) => {
                self.handle_commit(&text)?;
            }
            InputResult::Preedit(text, cursor) => {
                self.handle_preedit(&text, cursor)?;
            }
            InputResult::DeleteSurrounding(before, after) => {
                self.handle_delete_surrounding(before, after)?;
            }
            InputResult::None => {
                // No action needed
            }
        }
        
        Ok(())
    }
    
    /// Handle commit text
    fn handle_commit(&mut self, text: &str) -> XimResult<()> {
        if let Some(ref mut context) = self.input_context {
            context.entry.set_text(text);
            context.state = GtkInputMethodState::Committed;
            info!("GTK3 text committed: {}", text);
        }
        
        Ok(())
    }
    
    /// Handle preedit text
    fn handle_preedit(&mut self, text: &str, cursor: i32) -> XimResult<()> {
        if let Some(ref mut context) = self.input_context {
            context.preedit_text = text.to_string();
            context.cursor_position = cursor;
            context.state = GtkInputMethodState::Preedit;
            
            // Update entry with preedit text
            let current_text = context.entry.text();
            let new_text = format!("{}{}", current_text, text);
            context.entry.set_text(&new_text);
            
            info!("GTK3 preedit text: {} (cursor: {})", text, cursor);
        }
        
        Ok(())
    }
    
    /// Handle delete surrounding text
    fn handle_delete_surrounding(&mut self, before: u32, after: u32) -> XimResult<()> {
        if let Some(ref mut context) = self.input_context {
            let current_text = context.entry.text();
            let text_len = current_text.len() as u32;
            
            if before + after < text_len {
                let start = before as usize;
                let end = text_len as usize - after as usize;
                let new_text = format!("{}{}", 
                    &current_text[..start], 
                    &current_text[end..]
                );
                context.entry.set_text(&new_text);
                
                info!("GTK3 deleted surrounding text: {} chars before, {} chars after", before, after);
            }
        }
        
        Ok(())
    }
    
    /// Convert GTK key to KeyEvent
    fn gtk_key_to_key_event(&self, key: u32, state: u32, time: u32) -> XimResult<KeyEvent> {
        // Convert GTK key to character
        let character = self.gtk_key_to_character(key)?;
        
        // Determine key type
        let event_type = if state & 1 != 0 {
            KeyEventType::Normal  // Key release
        } else {
            KeyEventType::Normal  // Key press
        };
        
        // Convert character to VnLexiName
        let vn_sym = match character {
            'a' => VnLexiName::a,
            'b' => VnLexiName::b,
            'c' => VnLexiName::c,
            'd' => VnLexiName::d,
            'e' => VnLexiName::e,
            'f' => VnLexiName::f,
            'g' => VnLexiName::g,
            'h' => VnLexiName::h,
            'i' => VnLexiName::i,
            'j' => VnLexiName::j,
            'k' => VnLexiName::k,
            'l' => VnLexiName::l,
            'm' => VnLexiName::m,
            'n' => VnLexiName::n,
            'o' => VnLexiName::o,
            'p' => VnLexiName::p,
            'q' => VnLexiName::q,
            'r' => VnLexiName::r,
            's' => VnLexiName::s,
            't' => VnLexiName::t,
            'u' => VnLexiName::u,
            'v' => VnLexiName::v,
            'w' => VnLexiName::w,
            'x' => VnLexiName::x,
            'y' => VnLexiName::y,
            'z' => VnLexiName::z,
            ' ' => VnLexiName::NonVnChar,
            _ => VnLexiName::NonVnChar,
        };
        
        Ok(KeyEvent {
            event_type,
            char_type: CharType::Vn,
            vn_sym,
            tone: 0,
            key_code: key,
            modifiers: self.gtk_state_to_modifiers(state),
        })
    }
    
    /// Convert GTK key to character
    fn gtk_key_to_character(&self, key: u32) -> XimResult<char> {
        // Basic key to character mapping
        let character = match key {
            0x20 => ' ',  // Space
            0x61..=0x7A => (key as u8) as char,  // a-z
            0x41..=0x5A => (key as u8) as char,  // A-Z
            0x30..=0x39 => (key as u8) as char,  // 0-9
            _ => return Err(XimError::GtkError(format!("Unsupported key: {}", key))),
        };
        
        Ok(character)
    }
    
    /// Convert GTK state to modifiers
    fn gtk_state_to_modifiers(&self, state: u32) -> u32 {
        let mut modifiers = 0;
        
        if state & (1 << 0) != 0 { modifiers |= 0x01; } // Shift
        if state & (1 << 1) != 0 { modifiers |= 0x02; } // Ctrl
        if state & (1 << 2) != 0 { modifiers |= 0x04; } // Alt
        if state & (1 << 3) != 0 { modifiers |= 0x08; } // Super
        
        modifiers
    }
    
    /// Start input method
    pub fn start(&mut self) -> XimResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        info!("GTK3 input method started");
        Ok(())
    }
    
    /// Stop input method
    pub fn stop(&mut self) -> XimResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("GTK3 input method stopped");
        Ok(())
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    /// Get configuration
    pub fn config(&self) -> &GtkInputMethodConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: GtkInputMethodConfig) {
        self.config = config;
        info!("GTK3 input method configuration updated");
    }
}

impl GtkInputContext {
    /// Create a new GTK3 input context
    pub fn new(entry: Entry) -> Self {
        Self {
            entry,
            preedit_text: String::new(),
            cursor_position: 0,
            state: GtkInputMethodState::Inactive,
            key_buffer: Vec::new(),
        }
    }
    
    /// Get preedit text
    pub fn preedit_text(&self) -> &str {
        &self.preedit_text
    }
    
    /// Get cursor position
    pub fn cursor_position(&self) -> i32 {
        self.cursor_position
    }
    
    /// Get state
    pub fn state(&self) -> &GtkInputMethodState {
        &self.state
    }
    
    /// Set preedit text
    pub fn set_preedit_text(&mut self, text: String, cursor: i32) {
        self.preedit_text = text;
        self.cursor_position = cursor;
        self.state = GtkInputMethodState::Preedit;
    }
    
    /// Clear preedit text
    pub fn clear_preedit(&mut self) {
        self.preedit_text.clear();
        self.cursor_position = 0;
        self.state = GtkInputMethodState::Inactive;
    }
    
    /// Commit preedit text
    pub fn commit_preedit(&mut self) {
        self.preedit_text.clear();
        self.cursor_position = 0;
        self.state = GtkInputMethodState::Committed;
    }
}

impl Clone for GtkInputContext {
    fn clone(&self) -> Self {
        Self {
            entry: self.entry.clone(),
            preedit_text: self.preedit_text.clone(),
            cursor_position: self.cursor_position,
            state: self.state.clone(),
            key_buffer: self.key_buffer.clone(),
        }
    }
}

impl GtkInputMethodManager {
    /// Create a new GTK3 input method manager
    pub fn new() -> Self {
        Self {
            module: Arc::new(RwLock::new(GtkInputMethodModule::new())),
            event_handlers: Vec::new(),
            input_contexts: HashMap::new(),
            config: GtkInputMethodConfig::default(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) -> XimResult<()> {
        info!("Initializing GTK3 input method manager...");
        
        // Initialize GTK module
        {
            let mut module = self.module.write().unwrap();
            module.initialize()?;
        }
        
        info!("GTK3 input method manager initialized successfully");
        Ok(())
    }
    
    /// Add event handler
    pub fn add_event_handler(&mut self, handler: Box<dyn GtkInputMethodEventHandler>) {
        self.event_handlers.push(handler);
        info!("GTK3 event handler added");
    }
    
    /// Create input context
    pub fn create_input_context(&mut self, id: String, entry: Entry) -> XimResult<()> {
        let context = GtkInputContext::new(entry);
        self.input_contexts.insert(id.clone(), context);
        info!("GTK3 input context created: {}", id);
        Ok(())
    }
    
    /// Remove input context
    pub fn remove_input_context(&mut self, id: &str) -> XimResult<()> {
        if self.input_contexts.remove(id).is_some() {
            info!("GTK3 input context removed: {}", id);
        } else {
            return Err(XimError::GtkError(format!("Input context not found: {}", id)));
        }
        Ok(())
    }
    
    /// Process key event
    pub fn process_key_event(&mut self, context_id: &str, key: u32, state: u32, time: u32) -> XimResult<()> {
        if let Some(_context) = self.input_contexts.get(context_id) {
            let mut module = self.module.write().unwrap();
            module.process_key_event(key, state, time)?;
        } else {
            return Err(XimError::GtkError(format!("Input context not found: {}", context_id)));
        }
        Ok(())
    }
    
    /// Set input method engine
    pub fn set_engine(&mut self, engine: Box<dyn InputMethodTrait + Send + Sync>) -> XimResult<()> {
        let mut module = self.module.write().unwrap();
        module.set_engine(engine);
        Ok(())
    }
    
    /// Start manager
    pub fn start(&mut self) -> XimResult<()> {
        let mut module = self.module.write().unwrap();
        module.start()?;
        info!("GTK3 input method manager started");
        Ok(())
    }
    
    /// Stop manager
    pub fn stop(&mut self) -> XimResult<()> {
        let mut module = self.module.write().unwrap();
        module.stop()?;
        info!("GTK3 input method manager stopped");
        Ok(())
    }
    
    /// Get configuration
    pub fn config(&self) -> &GtkInputMethodConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: GtkInputMethodConfig) {
        self.config = config.clone();
        let mut module = self.module.write().unwrap();
        module.set_config(config);
    }
}

/// GTK3 Input Method Demo Handler
pub struct GtkInputMethodDemoHandler {
    /// Handler name
    name: String,
}

impl GtkInputMethodDemoHandler {
    /// Create a new demo handler
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl GtkInputMethodEventHandler for GtkInputMethodDemoHandler {
    fn handle_event(&mut self, event: GtkInputMethodEvent) -> XimResult<()> {
        info!("GTK3 Demo Handler '{}' received event: {:?}", self.name, event);
        Ok(())
    }
    
    fn handle_preedit(&mut self, text: &str, cursor: i32) -> XimResult<()> {
        info!("GTK3 Demo Handler '{}' preedit: '{}' (cursor: {})", self.name, text, cursor);
        Ok(())
    }
    
    fn handle_commit(&mut self, text: &str) -> XimResult<()> {
        info!("GTK3 Demo Handler '{}' commit: '{}'", self.name, text);
        Ok(())
    }
    
    fn handle_input_method_change(&mut self, method: &str) -> XimResult<()> {
        info!("GTK3 Demo Handler '{}' input method changed to: {}", self.name, method);
        Ok(())
    }
}