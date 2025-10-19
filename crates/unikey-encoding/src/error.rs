//! Error types for encoding

use thiserror::Error;

/// Encoding error types
#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("Invalid character set: {0}")]
    InvalidCharset(String),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
    
    #[error("Unicode error: {0}")]
    UnicodeError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, EncodingError>;
