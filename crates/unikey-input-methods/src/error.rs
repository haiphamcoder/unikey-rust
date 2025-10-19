//! Error types for input methods

use thiserror::Error;

/// Input method error types
#[derive(Error, Debug)]
pub enum InputMethodError {
    #[error("Invalid key event: {0}")]
    InvalidKeyEvent(String),
    
    #[error("Method error: {0}")]
    MethodError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, InputMethodError>;
