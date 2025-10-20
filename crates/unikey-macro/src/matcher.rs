//! Pattern matching utilities for macros

use crate::MacroError;

/// Pattern matcher for macro definitions
#[derive(Debug, Clone)]
pub struct PatternMatcher {
    /// Compiled pattern
    pattern: String,
    /// Whether the pattern is case-sensitive
    case_sensitive: bool,
    /// Whether to match whole words only
    whole_word: bool,
}

impl PatternMatcher {
    /// Create a new pattern matcher
    pub fn new(pattern: String, case_sensitive: bool, whole_word: bool) -> Self {
        Self {
            pattern,
            case_sensitive,
            whole_word,
        }
    }

    /// Check if the pattern matches the given text
    pub fn matches(&self, text: &str) -> bool {
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
                if pattern.contains('*') || pattern.contains('?') {
                    self.match_pattern(word, &pattern)
                } else {
                    word == pattern
                }
            })
        } else {
            // Match anywhere in text
            if pattern.contains('*') || pattern.contains('?') {
                self.match_pattern(&text_to_match, &pattern)
            } else {
                text_to_match.contains(&pattern)
            }
        }
    }

    /// Find all matches in the text
    pub fn find_all_matches(&self, text: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let pattern = if self.case_sensitive {
            self.pattern.clone()
        } else {
            self.pattern.to_lowercase()
        };

        let text_to_search = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        if self.whole_word {
            // Find whole word matches
            let mut start = 0;
            for word in text_to_search.split_whitespace() {
                if self.match_pattern(word, &pattern) {
                    let word_start = text_to_search[start..].find(word).unwrap() + start;
                    matches.push(Match {
                        start: word_start,
                        end: word_start + word.len(),
                        matched_text: word.to_string(),
                    });
                }
                start += word.len() + 1; // +1 for space
            }
        } else {
            // Find substring matches
            if pattern.contains('*') || pattern.contains('?') {
                // TODO: Implement wildcard matching for find_all
                // For now, just do simple string search
                let mut start = 0;
                while let Some(pos) = text_to_search[start..].find(&pattern) {
                    let actual_pos = start + pos;
                    matches.push(Match {
                        start: actual_pos,
                        end: actual_pos + pattern.len(),
                        matched_text: pattern.clone(),
                    });
                    start = actual_pos + 1;
                }
            } else {
                let mut start = 0;
                while let Some(pos) = text_to_search[start..].find(&pattern) {
                    let actual_pos = start + pos;
                    matches.push(Match {
                        start: actual_pos,
                        end: actual_pos + pattern.len(),
                        matched_text: pattern.clone(),
                    });
                    start = actual_pos + 1;
                }
            }
        }

        matches
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
}

/// Represents a match found in text
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// Start position of the match
    pub start: usize,
    /// End position of the match
    pub end: usize,
    /// The matched text
    pub matched_text: String,
}

/// Advanced pattern matching with regex-like features
#[derive(Debug, Clone)]
pub struct AdvancedMatcher {
    /// Pattern with advanced features
    pattern: String,
    /// Whether the pattern is case-sensitive
    case_sensitive: bool,
}

impl AdvancedMatcher {
    /// Create a new advanced matcher
    pub fn new(pattern: String, case_sensitive: bool) -> Self {
        Self {
            pattern,
            case_sensitive,
        }
    }

    /// Check if the pattern matches the given text
    pub fn matches(&self, text: &str) -> bool {
        // TODO: Implement advanced pattern matching
        // For now, just do simple string matching
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

        text_to_match.contains(&pattern)
    }

    /// Find all matches with capture groups
    pub fn find_all_with_captures(&self, text: &str) -> Vec<CaptureMatch> {
        // TODO: Implement capture group matching
        // For now, return empty vector
        Vec::new()
    }
}

/// Represents a match with capture groups
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureMatch {
    /// The full match
    pub full_match: Match,
    /// Capture groups
    pub captures: Vec<Option<String>>,
}
