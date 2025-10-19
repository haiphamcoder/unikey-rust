//! # UniKey Core
//! 
//! Core engine and input processing for UniKey Vietnamese input method.
//! 
//! This crate provides the fundamental data structures, algorithms, and processing
//! pipeline for Vietnamese text input. It handles key events, character generation,
//! and state management.

pub mod types;
pub mod engine;
pub mod processor;
pub mod state;
pub mod error;
pub mod mappings;

pub use types::*;
pub use engine::*;
pub use processor::*;
pub use state::*;
pub use error::*;
pub use mappings::*;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Core functionality re-exports
pub mod prelude {
    pub use crate::{
        types::*,
        engine::*,
        processor::*,
        state::*,
        error::*,
        mappings::*,
    };
}
