//! System tray implementation for Windows

/// Windows system tray
pub struct WindowsTray;

impl WindowsTray {
    /// Create a new Windows system tray
    pub fn new() -> Self {
        Self
    }
    
    /// Show tray icon
    pub fn show(&mut self) -> Result<(), String> {
        // TODO: Implement Windows system tray
        Ok(())
    }
}
