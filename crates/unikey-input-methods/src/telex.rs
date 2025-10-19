//! Telex input method

use crate::traits::InputMethod;
use unikey_core::KeyEvent;

/// Telex input method
pub struct TelexMethod;

impl TelexMethod {
    /// Create a new Telex method
    pub fn new() -> Self {
        Self
    }
}

impl InputMethod for TelexMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        // TODO: Implement Telex processing
        Ok(vec![])
    }
    
    fn reset(&mut self) {
        // TODO: Implement reset
    }
    
    fn name(&self) -> &str {
        "Telex"
    }
}
