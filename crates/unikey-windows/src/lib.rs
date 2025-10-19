//! # UniKey Windows
//! 
//! Windows platform integration for UniKey.
//! 
//! This crate provides Windows-specific implementations including IME framework,
//! system tray integration, and Windows-specific optimizations.

pub mod ime;
pub mod tray;
pub mod registry;
pub mod error;

pub use ime::*;
pub use tray::*;
pub use registry::*;
pub use error::*;

/// Windows platform functionality re-exports
pub mod prelude {
    pub use crate::{
        ime::*,
        tray::*,
        registry::*,
        error::*,
    };
}
