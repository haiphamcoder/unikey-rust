//! Error types for input methods

use thiserror::Error;

/// Input method errors
#[derive(Error, Debug)]
pub enum InputMethodError {
    #[error("Invalid character sequence: {0}")]
    InvalidSequence(String),
    
    #[error("Unsupported input method: {0}")]
    UnsupportedMethod(String),
    
    #[error("Character mapping not found: {0}")]
    MappingNotFound(String),
    
    #[error("Buffer overflow: maximum length exceeded")]
    BufferOverflow,
    
    #[error("Invalid tone mark: {0}")]
    InvalidTone(u8),
    
    #[error("Invalid diacritic: {0}")]
    InvalidDiacritic(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

/// Result type for input methods
pub type Result<T> = std::result::Result<T, InputMethodError>;