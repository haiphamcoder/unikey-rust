//! Error types for Windows platform

use thiserror::Error;

/// Windows platform error types
#[derive(Error, Debug)]
pub enum WindowsError {
    #[error("IME error: {0}")]
    ImeError(String),
    
    #[error("System tray error: {0}")]
    TrayError(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, WindowsError>;
