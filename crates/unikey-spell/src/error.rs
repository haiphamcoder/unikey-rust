//! Error types for the spell checking system

use thiserror::Error;

/// Errors that can occur in the spell checking system
#[derive(Error, Debug)]
pub enum SpellError {
    #[error("Dictionary error: {0}")]
    DictionaryError(String),

    #[error("Detection error: {0}")]
    DetectionError(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Free marking error: {0}")]
    FreeMarkingError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}

/// Result type for spell checking operations
pub type SpellResult<T> = Result<T, SpellError>;
