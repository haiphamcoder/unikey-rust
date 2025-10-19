//! # UniKey Input Methods
//! 
//! Input method implementations for UniKey Vietnamese input method.
//! 
//! This crate provides implementations of various Vietnamese input methods
//! including Telex, VNI, VIQR, and user-defined methods.

pub mod traits;
pub mod telex;
pub mod vni;
pub mod viqr;
pub mod user;
pub mod error;

pub use traits::*;
pub use telex::*;
pub use vni::*;
pub use viqr::*;
pub use user::*;
pub use error::*;

/// Input method functionality re-exports
pub mod prelude {
    pub use crate::{
        traits::*,
        telex::*,
        vni::*,
        viqr::*,
        user::*,
        error::*,
    };
}
