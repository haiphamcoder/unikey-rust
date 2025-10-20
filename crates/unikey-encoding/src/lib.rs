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
pub mod standard_mappings;

// Legacy encoding modules
pub mod tcvn3;
pub mod tcvn3_simple;
pub mod vps;
pub mod viscii;
pub mod vni_win;
pub mod bkhcm;
pub mod cp1258;

pub use charset::*;
pub use converter::*;
pub use unicode::*;
pub use legacy::*;
pub use error::*;
pub use standard_mappings::*;

// Legacy encoding re-exports
pub use tcvn3::*;
pub use tcvn3_simple::*;
pub use vps::*;
pub use viscii::*;
pub use vni_win::*;
pub use bkhcm::*;
pub use cp1258::*;

/// Encoding functionality re-exports
pub mod prelude {
    pub use crate::{
        charset::*,
        converter::*,
        unicode::*,
        legacy::*,
        error::*,
        tcvn3::*,
        vps::*,
        viscii::*,
        vni_win::*,
        bkhcm::*,
        cp1258::*,
    };
}
