//! Error types for macOS platform

use thiserror::Error;

/// macOS platform error types
#[derive(Error, Debug)]
pub enum MacosError {
    #[error("Input method error: {0}")]
    InputMethodError(String),
    
    #[error("Preferences error: {0}")]
    PreferencesError(String),
    
    #[error("UI error: {0}")]
    UiError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, MacosError>;
