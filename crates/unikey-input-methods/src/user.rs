//! User-defined input method implementation

use crate::traits::InputMethod;
use unikey_core::KeyEvent;
use std::collections::HashMap;

/// User-defined input method
pub struct UserMethod {
    /// Custom character mappings
    mappings: HashMap<String, char>,
    /// Method name
    name: String,
}

impl UserMethod {
    /// Create a new user-defined method
    pub fn new(name: String) -> Self {
        Self {
            mappings: HashMap::new(),
            name,
        }
    }
    
    /// Add a custom mapping
    pub fn add_mapping(&mut self, sequence: String, character: char) {
        self.mappings.insert(sequence, character);
    }
    
    /// Remove a custom mapping
    pub fn remove_mapping(&mut self, sequence: &str) {
        self.mappings.remove(sequence);
    }
    
    /// Get all mappings
    pub fn get_mappings(&self) -> &HashMap<String, char> {
        &self.mappings
    }
}

impl InputMethod for UserMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        // TODO: Implement user-defined processing
        Ok(vec![event.key_code as u8])
    }
    
    fn reset(&mut self) {
        // TODO: Implement reset
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl Default for UserMethod {
    fn default() -> Self {
        Self::new("User".to_string())
    }
}