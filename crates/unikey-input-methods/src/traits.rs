//! Input method traits

use unikey_core::KeyEvent;

/// Input method trait
pub trait InputMethod {
    /// Process a key event
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String>;
    
    /// Reset method state
    fn reset(&mut self);
    
    /// Get method name
    fn name(&self) -> &str;
}
