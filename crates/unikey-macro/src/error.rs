//! Error types for macro

use thiserror::Error;

/// Macro error types
#[derive(Error, Debug)]
pub enum MacroError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, MacroError>;
