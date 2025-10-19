//! IME implementation for Windows

/// Windows IME
pub struct WindowsIme;

impl WindowsIme {
    /// Create a new Windows IME
    pub fn new() -> Self {
        Self
    }
    
    /// Start the IME
    pub fn start(&mut self) -> Result<(), String> {
        // TODO: Implement Windows IME
        Ok(())
    }
}
