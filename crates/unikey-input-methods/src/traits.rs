//! Input method traits

use unikey_core::KeyEvent;

/// Input method types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum InputMethodType {
    Telex,
    VNI,
    VIQR,
    User,
}

impl std::fmt::Display for InputMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMethodType::Telex => write!(f, "Telex"),
            InputMethodType::VNI => write!(f, "VNI"),
            InputMethodType::VIQR => write!(f, "VIQR"),
            InputMethodType::User => write!(f, "User"),
        }
    }
}

/// Input method trait
pub trait InputMethod {
    /// Process a key event
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String>;
    
    /// Reset method state
    fn reset(&mut self);
    
    /// Get method name
    fn name(&self) -> &str;
}
