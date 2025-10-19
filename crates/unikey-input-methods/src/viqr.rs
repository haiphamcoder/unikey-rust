//! VIQR input method

use crate::traits::InputMethod;
use unikey_core::KeyEvent;

/// VIQR input method
pub struct ViqrMethod;

impl ViqrMethod {
    /// Create a new VIQR method
    pub fn new() -> Self {
        Self
    }
}

impl InputMethod for ViqrMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        // TODO: Implement VIQR processing
        Ok(vec![])
    }
    
    fn reset(&mut self) {
        // TODO: Implement reset
    }
    
    fn name(&self) -> &str {
        "VIQR"
    }
}
