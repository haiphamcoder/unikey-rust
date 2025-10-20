//! UniKey Linux XIM Integration
//! 
//! This crate provides Linux XIM (X Input Method) integration for UniKey Rust,
//! allowing it to work as a system-wide input method on Linux desktop environments.
//! 
//! ## Features
//! 
//! - **XIM Server**: Full XIM server implementation
//! - **X11 Integration**: Native X11 window system integration
//! - **Input Method Switching**: Dynamic switching between input methods
//! - **Configuration**: Runtime configuration and hotkey management
//! - **System Integration**: Desktop environment integration
//! 
//! ## Usage
//! 
//! ```rust
//! use unikey_linux::{XimServer, XimConfig};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = XimConfig::default();
//!     let mut server = XimServer::new(config).await?;
//!     
//!     // Start the XIM server
//!     server.run().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod server;
pub mod client;
pub mod protocol;
pub mod config;
pub mod error;
pub mod x11_utils;

pub use server::*;
pub use client::*;
pub use protocol::*;
pub use config::*;
pub use error::*;
pub use x11_utils::*;

/// Linux XIM functionality re-exports
pub mod prelude {
    pub use crate::{
        server::*,
        client::*,
        protocol::*,
        config::*,
        error::*,
        x11_utils::*,
    };
}