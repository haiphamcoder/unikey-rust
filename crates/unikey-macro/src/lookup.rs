//! Macro lookup

use crate::definition::MacroDefinition;

/// Macro lookup
pub struct MacroLookup {
    macros: Vec<MacroDefinition>,
}

impl MacroLookup {
    /// Create a new macro lookup
    pub fn new() -> Self {
        Self {
            macros: Vec::new(),
        }
    }
    
    /// Lookup macro by key
    pub fn lookup(&self, key: &str) -> Option<&MacroDefinition> {
        self.macros.iter().find(|m| m.key == key)
    }
}
