//! Error types for platform common

use thiserror::Error;

/// Platform common error types
#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Initialization error: {0}")]
    InitializationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("UI error: {0}")]
    UiError(String),
    
    #[error("Event error: {0}")]
    EventError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, PlatformError>;
