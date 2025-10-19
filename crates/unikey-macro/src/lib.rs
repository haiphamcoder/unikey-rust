//! # UniKey Macro
//! 
//! Macro system for UniKey Vietnamese input method.
//! 
//! This crate provides macro definition, storage, lookup, and replacement
//! functionality for text shortcuts and abbreviations.

pub mod definition;
pub mod storage;
pub mod lookup;
pub mod io;
pub mod error;

pub use definition::*;
pub use storage::*;
pub use lookup::*;
pub use io::*;
pub use error::*;

/// Macro functionality re-exports
pub mod prelude {
    pub use crate::{
        definition::*,
        storage::*,
        lookup::*,
        io::*,
        error::*,
    };
}
