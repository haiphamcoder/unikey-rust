//! Platform common traits

/// Platform trait
pub trait Platform {
    /// Initialize platform
    fn init(&mut self) -> Result<(), String>;
    
    /// Shutdown platform
    fn shutdown(&mut self) -> Result<(), String>;
    
    /// Get platform name
    fn name(&self) -> &str;
}
