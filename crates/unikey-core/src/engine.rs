//! Core engine for UniKey

use crate::{KeyEvent, InputMethod, OutputType, Result};

/// Main UniKey engine
pub struct UnikeyEngine {
    input_method: InputMethod,
    output_type: OutputType,
    enabled: bool,
}

impl UnikeyEngine {
    /// Create a new UniKey engine
    pub fn new() -> Self {
        Self {
            input_method: InputMethod::Telex,
            output_type: OutputType::Unicode,
            enabled: true,
        }
    }
    
    /// Process a key event
    pub fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>> {
        if !self.enabled {
            return Ok(vec![]);
        }
        
        // TODO: Implement key processing logic
        Ok(vec![])
    }
    
    /// Set input method
    pub fn set_input_method(&mut self, method: InputMethod) {
        self.input_method = method;
    }
    
    /// Set output type
    pub fn set_output_type(&mut self, output_type: OutputType) {
        self.output_type = output_type;
    }
    
    /// Enable/disable engine
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Check if engine is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for UnikeyEngine {
    fn default() -> Self {
        Self::new()
    }
}
