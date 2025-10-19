//! Macro definition

/// Macro definition
#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub key: String,
    pub text: String,
}

impl MacroDefinition {
    /// Create a new macro definition
    pub fn new(key: String, text: String) -> Self {
        Self { key, text }
    }
}
