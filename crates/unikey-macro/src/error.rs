//! Error types for the macro system

use thiserror::Error;

/// Errors that can occur in the macro system
#[derive(Error, Debug)]
pub enum MacroError {
    #[error("Invalid macro definition: {0}")]
    InvalidMacro(String),

    #[error("Macro not found: {0}")]
    MacroNotFound(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Pattern matching error: {0}")]
    PatternError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type for macro operations
pub type MacroResult<T> = Result<T, MacroError>;