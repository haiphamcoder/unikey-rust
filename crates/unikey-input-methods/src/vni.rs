//! VNI input method

use crate::traits::InputMethod;
use unikey_core::KeyEvent;

/// VNI input method
pub struct VniMethod;

impl VniMethod {
    /// Create a new VNI method
    pub fn new() -> Self {
        Self
    }
}

impl InputMethod for VniMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        // TODO: Implement VNI processing
        Ok(vec![])
    }
    
    fn reset(&mut self) {
        // TODO: Implement reset
    }
    
    fn name(&self) -> &str {
        "VNI"
    }
}
