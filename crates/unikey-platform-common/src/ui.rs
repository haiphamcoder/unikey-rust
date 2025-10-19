//! Platform UI

/// UI trait
pub trait PlatformUI {
    /// Show status
    fn show_status(&self, message: &str);
    
    /// Hide status
    fn hide_status(&self);
    
    /// Show error
    fn show_error(&self, error: &str);
}
