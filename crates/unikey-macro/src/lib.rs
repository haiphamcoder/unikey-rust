//! UniKey Macro System
//! 
//! This crate provides a comprehensive macro system for UniKey Rust,
//! allowing users to define custom text replacements and shortcuts.
//! 
//! ## Features
//! 
//! - **Macro Definition**: Define custom text replacements
//! - **Pattern Matching**: Support for complex pattern matching
//! - **Context Awareness**: Macros can be context-sensitive
//! - **Persistence**: Save and load macro definitions
//! - **Import/Export**: Share macro definitions between users
//! 
//! ## Usage
//! 
//! ```rust
//! use unikey_macro::{MacroEngine, MacroDefinition};
//! 
//! let mut engine = MacroEngine::new();
//! 
//! // Define a simple macro
//! let macro_def = MacroDefinition::new("hello", "Xin chào!");
//! engine.add_macro(macro_def);
//! 
//! // Process text with macros
//! let result = engine.process_text("hello world");
//! assert_eq!(result, "Xin chào! world");
//! ```

pub mod engine;
pub mod definition;
pub mod matcher;
pub mod storage;
pub mod error;

pub use engine::*;
pub use definition::*;
pub use matcher::*;
pub use storage::*;
pub use error::*;

/// Macro system functionality re-exports
pub mod prelude {
    pub use crate::{
        engine::*,
        definition::*,
        matcher::*,
        storage::*,
        error::*,
    };
}