//! Macro storage

use crate::definition::MacroDefinition;

/// Macro storage
pub struct MacroStorage {
    macros: Vec<MacroDefinition>,
}

impl MacroStorage {
    /// Create a new macro storage
    pub fn new() -> Self {
        Self {
            macros: Vec::new(),
        }
    }
    
    /// Add a macro
    pub fn add_macro(&mut self, macro_def: MacroDefinition) {
        self.macros.push(macro_def);
    }
    
    /// Get all macros
    pub fn get_macros(&self) -> &[MacroDefinition] {
        &self.macros
    }
}
