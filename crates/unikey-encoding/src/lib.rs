//! # UniKey Encoding
//! 
//! Character set conversion and encoding support for UniKey.
//! 
//! This crate provides comprehensive support for Vietnamese character encoding
//! and conversion between different character sets including Unicode, TCVN3,
//! VNI, VIQR, and other legacy encodings.

pub mod charset;
pub mod converter;
pub mod unicode;
pub mod legacy;
pub mod error;

pub use charset::*;
pub use converter::*;
pub use unicode::*;
pub use legacy::*;
pub use error::*;

/// Encoding functionality re-exports
pub mod prelude {
    pub use crate::{
        charset::*,
        converter::*,
        unicode::*,
        legacy::*,
        error::*,
    };
}
