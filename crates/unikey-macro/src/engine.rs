//! Macro engine for processing text with macro replacements

use crate::{MacroCollection, MacroDefinition, MacroError, MacroResult};
use std::collections::HashMap;

/// Main macro engine that processes text and applies macro replacements
#[derive(Debug, Clone)]
pub struct MacroEngine {
    /// Collection of macro definitions
    collection: MacroCollection,
    /// Statistics for performance monitoring
    stats: MacroStats,
}

/// Statistics for macro processing
#[derive(Debug, Clone, Default)]
pub struct MacroStats {
    /// Total number of macro applications
    pub total_applications: u64,
    /// Number of successful applications
    pub successful_applications: u64,
    /// Number of failed applications
    pub failed_applications: u64,
    /// Processing time in microseconds
    pub total_processing_time: u64,
}

impl MacroEngine {
    /// Create a new macro engine
    pub fn new() -> Self {
        Self {
            collection: MacroCollection::new(),
            stats: MacroStats::default(),
        }
    }

    /// Add a macro definition
    pub fn add_macro(&mut self, macro_def: MacroDefinition) -> MacroResult<()> {
        self.collection.add_macro(macro_def)
    }

    /// Remove a macro by ID
    pub fn remove_macro(&mut self, id: &str) -> Option<MacroDefinition> {
        self.collection.remove_macro(id)
    }

    /// Get a macro by ID
    pub fn get_macro(&self, id: &str) -> Option<&MacroDefinition> {
        self.collection.get_macro(id)
    }

    /// Get all macros
    pub fn get_all_macros(&self) -> &HashMap<String, MacroDefinition> {
        self.collection.get_all_macros()
    }

    /// Process text and apply macro replacements
    pub fn process_text(&mut self, text: &str) -> MacroResult<String> {
        let start_time = std::time::Instant::now();
        
        let mut result = text.to_string();
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10; // Prevent infinite loops
        
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;
            
            // Find all matching macros
            let matching_macros = self.collection.get_matching_macros(&result);
            
            for macro_def in matching_macros {
                let old_result = result.clone();
                result = self.apply_macro(&result, macro_def)?;
                
                if result != old_result {
                    changed = true;
                    self.stats.successful_applications += 1;
                    break; // Apply one macro per iteration to maintain order
                }
            }
        }
        
        if iterations >= MAX_ITERATIONS {
            self.stats.failed_applications += 1;
            return Err(MacroError::PatternError("Maximum iterations reached".to_string()));
        }
        
        self.stats.total_applications += 1;
        self.stats.total_processing_time += start_time.elapsed().as_micros() as u64;
        
        Ok(result)
    }

    /// Apply a single macro to text
    fn apply_macro(&self, text: &str, macro_def: &MacroDefinition) -> MacroResult<String> {
        if !macro_def.enabled {
            return Ok(text.to_string());
        }

        let pattern = if macro_def.case_sensitive {
            macro_def.pattern.clone()
        } else {
            macro_def.pattern.to_lowercase()
        };

        let text_to_process = if macro_def.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        if macro_def.whole_word {
            // Replace whole words only
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut result_words = Vec::new();
            
            for word in words {
                let word_to_match = if macro_def.case_sensitive {
                    word.to_string()
                } else {
                    word.to_lowercase()
                };
                
                if self.matches_word(&word_to_match, &pattern) {
                    result_words.push(macro_def.replacement.clone());
                } else {
                    result_words.push(word.to_string());
                }
            }
            
            Ok(result_words.join(" "))
        } else {
            // Replace anywhere in text
            if pattern.contains('*') || pattern.contains('?') {
                self.replace_pattern(text, &pattern, &macro_def.replacement)
            } else {
                Ok(text.replace(&pattern, &macro_def.replacement))
            }
        }
    }

    /// Check if a word matches a pattern
    fn matches_word(&self, word: &str, pattern: &str) -> bool {
        if pattern.contains('*') || pattern.contains('?') {
            self.match_pattern(word, pattern)
        } else {
            word == pattern
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
        if pattern_idx == pattern.len() {
            return text_idx == text.len();
        }

        if text_idx == text.len() {
            return pattern[pattern_idx..].iter().all(|&c| c == '*');
        }

        match pattern[pattern_idx] {
            '*' => {
                self.match_recursive(text, pattern, text_idx, pattern_idx + 1) ||
                self.match_recursive(text, pattern, text_idx + 1, pattern_idx)
            },
            '?' => {
                self.match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
            },
            c => {
                if text[text_idx] == c {
                    self.match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
                } else {
                    false
                }
            }
        }
    }

    /// Replace pattern with wildcards
    fn replace_pattern(&self, text: &str, pattern: &str, replacement: &str) -> MacroResult<String> {
        // For now, implement simple replacement
        // TODO: Implement proper wildcard replacement
        if pattern.contains('*') {
            // Simple implementation - replace first occurrence
            if let Some(pos) = text.find(pattern.trim_end_matches('*')) {
                let end_pos = pos + pattern.trim_end_matches('*').len();
                let mut result = text[..pos].to_string();
                result.push_str(replacement);
                result.push_str(&text[end_pos..]);
                return Ok(result);
            }
        }
        
        Ok(text.to_string())
    }

    /// Get processing statistics
    pub fn get_stats(&self) -> &MacroStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MacroStats::default();
    }

    /// Get macros by tag
    pub fn get_macros_by_tag(&self, tag: &str) -> Vec<&MacroDefinition> {
        self.collection.get_macros_by_tag(tag)
    }

    /// Get enabled macros only
    pub fn get_enabled_macros(&self) -> Vec<&MacroDefinition> {
        self.collection.get_enabled_macros()
    }

    /// Clear all macros
    pub fn clear(&mut self) {
        self.collection.clear();
    }

    /// Get the number of macros
    pub fn len(&self) -> usize {
        self.collection.len()
    }

    /// Check if the engine is empty
    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
}

impl Default for MacroEngine {
    fn default() -> Self {
        Self::new()
    }
}
