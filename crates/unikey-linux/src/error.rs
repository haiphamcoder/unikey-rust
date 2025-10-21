//! Error types for Linux XIM integration

use thiserror::Error;

/// Errors that can occur in Linux XIM integration
#[derive(Error, Debug)]
pub enum XimError {
    #[error("X11 connection error: {0}")]
    X11ConnectionError(String),

    #[error("XIM protocol error: {0}")]
    XimProtocolError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Input method error: {0}")]
    InputMethodError(String),

    #[error("Client connection error: {0}")]
    ClientConnectionError(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("X11 error: {0}")]
    X11Error(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("System error: {0}")]
    SystemError(#[from] nix::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Spell checking error: {0}")]
    SpellError(#[from] unikey_spell::SpellError),

    #[error("Wayland error: {0}")]
    WaylandError(String),

    #[error("GTK error: {0}")]
    GtkError(String),
}

/// Result type for XIM operations
pub type XimResult<T> = Result<T, XimError>;