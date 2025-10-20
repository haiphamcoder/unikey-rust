//! Free marking support for Vietnamese input

use crate::{SpellError, SpellResult, WordType};
use std::collections::HashMap;

/// Free marking mode configuration
#[derive(Debug, Clone, PartialEq)]
pub enum FreeMarkingMode {
    /// Disabled - normal input method behavior
    Disabled,
    /// Enabled - allow free marking of any text
    Enabled,
    /// Smart - enable only for non-Vietnamese text
    Smart,
}

/// Free marking processor
#[derive(Debug, Clone)]
pub struct FreeMarkingProcessor {
    /// Current free marking mode
    mode: FreeMarkingMode,
    /// Free marking patterns and their replacements
    patterns: HashMap<String, String>,
    /// Whether free marking is currently active
    active: bool,
}

impl FreeMarkingProcessor {
    /// Create a new free marking processor
    pub fn new() -> Self {
        Self {
            mode: FreeMarkingMode::Disabled,
            patterns: HashMap::new(),
            active: false,
        }
    }

    /// Create with a specific mode
    pub fn with_mode(mode: FreeMarkingMode) -> Self {
        Self {
            mode,
            patterns: HashMap::new(),
            active: false,
        }
    }

    /// Set the free marking mode
    pub fn set_mode(&mut self, mode: FreeMarkingMode) {
        self.mode = mode;
    }

    /// Get the current free marking mode
    pub fn get_mode(&self) -> &FreeMarkingMode {
        &self.mode
    }

    /// Check if free marking should be active for the given text
    pub fn should_activate(&self, text: &str, word_type: WordType) -> bool {
        match self.mode {
            FreeMarkingMode::Disabled => false,
            FreeMarkingMode::Enabled => true,
            FreeMarkingMode::Smart => {
                // Activate for non-Vietnamese text
                matches!(word_type, WordType::English | WordType::Mixed | WordType::Unknown)
            }
        }
    }

    /// Activate free marking
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate free marking
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Check if free marking is currently active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Add a free marking pattern
    pub fn add_pattern(&mut self, pattern: String, replacement: String) {
        self.patterns.insert(pattern, replacement);
    }

    /// Remove a free marking pattern
    pub fn remove_pattern(&mut self, pattern: &str) -> Option<String> {
        self.patterns.remove(pattern)
    }

    /// Get all free marking patterns
    pub fn get_patterns(&self) -> &HashMap<String, String> {
        &self.patterns
    }

    /// Process text with free marking
    pub fn process_text(&self, text: &str) -> SpellResult<String> {
        if !self.active {
            return Ok(text.to_string());
        }

        let mut result = text.to_string();
        
        // Apply free marking patterns
        for (pattern, replacement) in &self.patterns {
            result = result.replace(pattern, replacement);
        }

        Ok(result)
    }

    /// Clear all free marking patterns
    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }

    /// Get the number of free marking patterns
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Initialize with common free marking patterns
    pub fn with_common_patterns(mut self) -> Self {
        // Common abbreviations and shortcuts
        self.add_pattern("btw".to_string(), "by the way".to_string());
        self.add_pattern("lol".to_string(), "laugh out loud".to_string());
        self.add_pattern("omg".to_string(), "oh my god".to_string());
        self.add_pattern("wtf".to_string(), "what the f***".to_string());
        self.add_pattern("asap".to_string(), "as soon as possible".to_string());
        self.add_pattern("fyi".to_string(), "for your information".to_string());
        self.add_pattern("tbh".to_string(), "to be honest".to_string());
        self.add_pattern("imo".to_string(), "in my opinion".to_string());
        self.add_pattern("imho".to_string(), "in my humble opinion".to_string());
        self.add_pattern("idk".to_string(), "I don't know".to_string());
        self.add_pattern("idc".to_string(), "I don't care".to_string());
        self.add_pattern("irl".to_string(), "in real life".to_string());
        self.add_pattern("ttyl".to_string(), "talk to you later".to_string());
        self.add_pattern("brb".to_string(), "be right back".to_string());
        self.add_pattern("afk".to_string(), "away from keyboard".to_string());

        // Vietnamese abbreviations
        self.add_pattern("ko".to_string(), "không".to_string());
        self.add_pattern("dc".to_string(), "được".to_string());
        self.add_pattern("k".to_string(), "không".to_string());
        self.add_pattern("vs".to_string(), "với".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());
        self.add_pattern("cx".to_string(), "cũng".to_string());

        self
    }
}

impl Default for FreeMarkingProcessor {
    fn default() -> Self {
        Self::new()
    }
}
