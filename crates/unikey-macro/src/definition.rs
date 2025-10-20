//! Macro definition and management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::MacroError;

/// A macro definition containing pattern and replacement text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MacroDefinition {
    /// Unique identifier for the macro
    pub id: String,
    /// Pattern to match (can include wildcards)
    pub pattern: String,
    /// Replacement text
    pub replacement: String,
    /// Whether the macro is case-sensitive
    pub case_sensitive: bool,
    /// Whether the macro matches whole words only
    pub whole_word: bool,
    /// Priority for conflict resolution (higher = more priority)
    pub priority: u32,
    /// Whether the macro is enabled
    pub enabled: bool,
    /// Description of the macro
    pub description: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl MacroDefinition {
    /// Create a new macro definition
    pub fn new(id: String, pattern: String, replacement: String) -> Self {
        Self {
            id,
            pattern,
            replacement,
            case_sensitive: false,
            whole_word: false,
            priority: 0,
            enabled: true,
            description: None,
            tags: Vec::new(),
        }
    }

    /// Create a macro with case sensitivity
    pub fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Create a macro with whole word matching
    pub fn with_whole_word(mut self, whole_word: bool) -> Self {
        self.whole_word = whole_word;
        self
    }

    /// Create a macro with priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Create a macro with description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Create a macro with tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Validate the macro definition
    pub fn validate(&self) -> Result<(), MacroError> {
        if self.id.is_empty() {
            return Err(MacroError::InvalidMacro("Macro ID cannot be empty".to_string()));
        }
        
        if self.pattern.is_empty() {
            return Err(MacroError::InvalidMacro("Macro pattern cannot be empty".to_string()));
        }
        
        // Check for valid pattern characters
        if self.pattern.contains("**") {
            return Err(MacroError::InvalidMacro("Pattern cannot contain consecutive wildcards".to_string()));
        }
        
        Ok(())
    }

    /// Check if this macro matches the given text
    pub fn matches(&self, text: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let pattern = if self.case_sensitive {
            self.pattern.clone()
        } else {
            self.pattern.to_lowercase()
        };

        let text_to_match = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        if self.whole_word {
            // Match whole words only
            text_to_match.split_whitespace().any(|word| {
                if pattern.contains('*') {
                    self.match_pattern(word, &pattern)
                } else {
                    word == pattern
                }
            })
        } else {
            // Match anywhere in text
            if pattern.contains('*') {
                self.match_pattern(&text_to_match, &pattern)
            } else {
                text_to_match.contains(&pattern)
            }
        }
    }

    /// Match pattern with wildcard support
    fn match_pattern(&self, text: &str, pattern: &str) -> bool {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();
        
        self.match_recursive(&text_chars, &pattern_chars, 0, 0)
    }

    /// Recursive pattern matching with wildcards
    fn match_recursive(&self, text: &[char], pattern: &[char], text_idx: usize, pattern_idx: usize) -> bool {
        // If we've consumed all pattern characters
        if pattern_idx == pattern.len() {
            return text_idx == text.len();
        }

        // If we've consumed all text characters
        if text_idx == text.len() {
            return pattern[pattern_idx..].iter().all(|&c| c == '*');
        }

        match pattern[pattern_idx] {
            '*' => {
                // Wildcard: match zero or more characters
                self.match_recursive(text, pattern, text_idx, pattern_idx + 1) ||
                self.match_recursive(text, pattern, text_idx + 1, pattern_idx)
            },
            '?' => {
                // Single character wildcard
                self.match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
            },
            c => {
                // Regular character
                if text[text_idx] == c {
                    self.match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
                } else {
                    false
                }
            }
        }
    }
}

/// Collection of macro definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroCollection {
    /// Map of macro ID to definition
    macros: HashMap<String, MacroDefinition>,
    /// Next available priority
    next_priority: u32,
}

impl MacroCollection {
    /// Create a new empty macro collection
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
            next_priority: 0,
        }
    }

    /// Add a macro definition
    pub fn add_macro(&mut self, macro_def: MacroDefinition) -> Result<(), MacroError> {
        macro_def.validate()?;
        
        let id = macro_def.id.clone();
        let mut macro_def = macro_def;
        
        // Assign priority if not set
        if macro_def.priority == 0 {
            macro_def.priority = self.next_priority;
            self.next_priority += 1;
        }
        
        self.macros.insert(id, macro_def);
        Ok(())
    }

    /// Remove a macro by ID
    pub fn remove_macro(&mut self, id: &str) -> Option<MacroDefinition> {
        self.macros.remove(id)
    }

    /// Get a macro by ID
    pub fn get_macro(&self, id: &str) -> Option<&MacroDefinition> {
        self.macros.get(id)
    }

    /// Get all macros
    pub fn get_all_macros(&self) -> &HashMap<String, MacroDefinition> {
        &self.macros
    }

    /// Get macros that match the given text, sorted by priority
    pub fn get_matching_macros(&self, text: &str) -> Vec<&MacroDefinition> {
        let mut matching: Vec<&MacroDefinition> = self.macros
            .values()
            .filter(|macro_def| macro_def.matches(text))
            .collect();
        
        // Sort by priority (higher priority first)
        matching.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        matching
    }

    /// Get macros by tag
    pub fn get_macros_by_tag(&self, tag: &str) -> Vec<&MacroDefinition> {
        self.macros
            .values()
            .filter(|macro_def| macro_def.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get enabled macros only
    pub fn get_enabled_macros(&self) -> Vec<&MacroDefinition> {
        self.macros
            .values()
            .filter(|macro_def| macro_def.enabled)
            .collect()
    }

    /// Clear all macros
    pub fn clear(&mut self) {
        self.macros.clear();
        self.next_priority = 0;
    }

    /// Get the number of macros
    pub fn len(&self) -> usize {
        self.macros.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }
}

impl Default for MacroCollection {
    fn default() -> Self {
        Self::new()
    }
}