//! System tray implementation for Linux

/// System tray
pub struct SystemTray;

impl SystemTray {
    /// Create a new system tray
    pub fn new() -> Self {
        Self
    }
    
    /// Show tray icon
    pub fn show(&mut self) -> Result<(), String> {
        // TODO: Implement system tray
        Ok(())
    }
}
