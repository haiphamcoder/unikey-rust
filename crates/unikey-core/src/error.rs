//! Error types for UniKey core

use thiserror::Error;

/// Core error types
#[derive(Error, Debug)]
pub enum UnikeyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    #[error("Input method error: {0}")]
    InputMethodError(String),
    
    #[error("State error: {0}")]
    StateError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, UnikeyError>;
