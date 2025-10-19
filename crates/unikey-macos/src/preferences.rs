//! Preferences implementation for macOS

/// macOS preferences
pub struct MacosPreferences;

impl MacosPreferences {
    /// Create a new macOS preferences
    pub fn new() -> Self {
        Self
    }
    
    /// Set preference
    pub fn set_preference(&mut self, key: &str, value: &str) -> Result<(), String> {
        // TODO: Implement macOS preferences
        Ok(())
    }
}
