//! # UniKey macOS
//! 
//! macOS platform integration for UniKey.
//! 
//! This crate provides macOS-specific implementations including Input Method
//! framework, system preferences integration, and macOS-specific UI components.

pub mod input_method;
pub mod preferences;
pub mod ui;
pub mod error;

pub use input_method::*;
pub use preferences::*;
pub use ui::*;
pub use error::*;

/// macOS platform functionality re-exports
pub mod prelude {
    pub use crate::{
        input_method::*,
        preferences::*,
        ui::*,
        error::*,
    };
}
