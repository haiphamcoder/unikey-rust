//! UniKey Spell Checking and Smart Features
//! 
//! This crate provides spell checking and smart features for UniKey Rust,
//! including Vietnamese word detection, non-Vietnamese sequence handling,
//! and free marking support.
//! 
//! ## Features
//! 
//! - **Vietnamese Word Detection**: Identify Vietnamese words and sequences
//! - **Non-Vietnamese Handling**: Process non-Vietnamese text appropriately
//! - **Free Marking**: Support for free marking mode
//! - **Spell Checking**: Basic spell checking for Vietnamese text
//! - **Smart Processing**: Context-aware text processing
//! 
//! ## Usage
//! 
//! ```rust
//! use unikey_spell::{SpellChecker, WordType, ProcessingMode};
//! 
//! let mut checker = SpellChecker::new();
//! 
//! // Detect word type
//! let word_type = checker.detect_word_type("xin chào");
//! assert_eq!(word_type, WordType::Vietnamese);
//! 
//! // Process text with smart features
//! let result = checker.process_text("hello xin chào world", ProcessingMode::Smart);
//! ```

pub mod checker;
pub mod detector;
pub mod dictionary;
pub mod processor;
pub mod error;
pub mod free_marking;

pub use checker::*;
pub use detector::*;
pub use dictionary::*;
pub use processor::*;
pub use error::*;
pub use free_marking::*;

/// Spell checking functionality re-exports
pub mod prelude {
    pub use crate::{
        checker::*,
        detector::*,
        dictionary::*,
        processor::*,
        error::*,
        free_marking::*,
    };
}
