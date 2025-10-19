//! User-defined input method

use crate::traits::InputMethod;
use unikey_core::KeyEvent;

/// User-defined input method
pub struct UserMethod {
    name: String,
}

impl UserMethod {
    /// Create a new user method
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl InputMethod for UserMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        // TODO: Implement user-defined processing
        Ok(vec![])
    }
    
    fn reset(&mut self) {
        // TODO: Implement reset
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
