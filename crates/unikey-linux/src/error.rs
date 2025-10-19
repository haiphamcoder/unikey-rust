//! Error types for Linux platform

use thiserror::Error;

/// Linux platform error types
#[derive(Error, Debug)]
pub enum LinuxError {
    #[error("XIM error: {0}")]
    XimError(String),
    
    #[error("Wayland error: {0}")]
    WaylandError(String),
    
    #[error("GTK error: {0}")]
    GtkError(String),
    
    #[error("System tray error: {0}")]
    TrayError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, LinuxError>;
