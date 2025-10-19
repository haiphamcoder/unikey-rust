//! # UniKey Platform Common
//! 
//! Common platform abstractions for UniKey.
//! 
//! This crate provides platform-agnostic abstractions and utilities
//! that are shared across different platform implementations.

pub mod traits;
pub mod events;
pub mod config;
pub mod ui;
pub mod error;

pub use traits::*;
pub use events::*;
pub use config::*;
pub use ui::*;
pub use error::*;

/// Platform common functionality re-exports
pub mod prelude {
    pub use crate::{
        traits::*,
        events::*,
        config::*,
        ui::*,
        error::*,
    };
}
