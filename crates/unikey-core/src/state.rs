//! State management for UniKey

use crate::{InputMethod, OutputType};

/// UniKey state
#[derive(Debug, Clone)]
pub struct UnikeyState {
    pub input_method: InputMethod,
    pub output_type: OutputType,
    pub enabled: bool,
    pub buffer: Vec<u8>,
    pub last_key: Option<u32>,
}

impl UnikeyState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            input_method: InputMethod::Telex,
            output_type: OutputType::Unicode,
            enabled: true,
            buffer: Vec::new(),
            last_key: None,
        }
    }
    
    /// Reset state
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.last_key = None;
    }
    
    /// Add character to buffer
    pub fn add_char(&mut self, ch: u8) {
        self.buffer.push(ch);
    }
    
    /// Get buffer content
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
    
    /// Clear buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

impl Default for UnikeyState {
    fn default() -> Self {
        Self::new()
    }
}
