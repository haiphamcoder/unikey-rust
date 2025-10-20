//! Main spell checker interface

use crate::{SpellError, SpellResult, WordType, ProcessingMode, ProcessedText};
use crate::{WordDetector, VietnameseDictionary, SmartProcessor, FreeMarkingProcessor};

/// Main spell checker
#[derive(Debug, Clone)]
pub struct SpellChecker {
    /// Word detector
    detector: WordDetector,
    /// Vietnamese dictionary
    dictionary: VietnameseDictionary,
    /// Smart processor
    processor: SmartProcessor,
    /// Free marking processor
    free_marking: FreeMarkingProcessor,
}

impl SpellChecker {
    /// Create a new spell checker
    pub fn new() -> SpellResult<Self> {
        Ok(Self {
            detector: WordDetector::new()?,
            dictionary: VietnameseDictionary::with_common_words(),
            processor: SmartProcessor::new()?,
            free_marking: FreeMarkingProcessor::new(),
        })
    }

    /// Create with free marking enabled
    pub fn with_free_marking() -> SpellResult<Self> {
        Ok(Self {
            detector: WordDetector::new()?,
            dictionary: VietnameseDictionary::with_common_words(),
            processor: SmartProcessor::with_free_marking()?,
            free_marking: FreeMarkingProcessor::with_mode(crate::FreeMarkingMode::Smart)
                .with_common_patterns(),
        })
    }

    /// Detect word type
    pub fn detect_word_type(&self, word: &str) -> WordType {
        self.detector.detect_word_type(word)
    }

    /// Detect sequence type
    pub fn detect_sequence_type(&self, sequence: &str) -> WordType {
        self.detector.detect_sequence_type(sequence)
    }

    /// Process text with smart features
    pub fn process_text(&mut self, text: &str, mode: ProcessingMode) -> SpellResult<ProcessedText> {
        self.processor.set_mode(mode);
        self.processor.process_text(text)
    }

    /// Check if a word is spelled correctly
    pub fn is_spelled_correctly(&self, word: &str) -> bool {
        let word_type = self.detect_word_type(word);
        match word_type {
            WordType::Vietnamese => self.dictionary.contains(word),
            WordType::VietnamesePlain => {
                // Check if there's a version with diacritics
                self.dictionary.get_words()
                    .iter()
                    .any(|dict_word| self.remove_diacritics(dict_word) == word.to_lowercase())
            },
            WordType::English => {
                // For English words, we assume they're correct if they contain only English characters
                word.chars().all(|c| c.is_ascii_alphabetic())
            },
            _ => true, // Numbers, punctuation, etc. are considered correct
        }
    }

    /// Get suggestions for a misspelled word
    pub fn get_suggestions(&self, word: &str) -> Vec<String> {
        let word_type = self.detect_word_type(word);
        match word_type {
            WordType::Vietnamese => {
                // Suggest corrections for Vietnamese words
                let mut suggestions = Vec::new();
                let word_lower = word.to_lowercase();
                
                for dict_word in self.dictionary.get_words() {
                    if self.edit_distance(&word_lower, dict_word) <= 2 {
                        suggestions.push(dict_word.clone());
                    }
                }
                
                suggestions.sort_by(|a, b| {
                    let dist_a = self.edit_distance(&word_lower, a);
                    let dist_b = self.edit_distance(&word_lower, b);
                    dist_a.cmp(&dist_b)
                });
                
                suggestions.into_iter().take(5).collect()
            },
            WordType::VietnamesePlain => {
                // Suggest Vietnamese words with diacritics
                let word_lower = word.to_lowercase();
                let mut suggestions = Vec::new();
                
                for dict_word in self.dictionary.get_words() {
                    if self.remove_diacritics(dict_word) == word_lower {
                        suggestions.push(dict_word.clone());
                    }
                }
                
                suggestions
            },
            _ => Vec::new(),
        }
    }

    /// Add a Vietnamese word to the dictionary
    pub fn add_vietnamese_word(&mut self, word: String) {
        self.detector.add_vietnamese_word(word.clone());
        self.dictionary.add_word(word);
    }

    /// Add an English word to the dictionary
    pub fn add_english_word(&mut self, word: String) {
        self.detector.add_english_word(word);
    }

    /// Add a free marking pattern
    pub fn add_free_marking_pattern(&mut self, pattern: String, replacement: String) {
        self.free_marking.add_pattern(pattern, replacement);
    }

    /// Get statistics about the spell checker
    pub fn get_stats(&self) -> SpellCheckerStats {
        SpellCheckerStats {
            vietnamese_word_count: self.dictionary.word_count(),
            english_word_count: self.detector.english_word_count(),
            free_marking_pattern_count: self.free_marking.pattern_count(),
            total_word_count: self.dictionary.word_count() + self.detector.english_word_count(),
        }
    }

    /// Remove diacritics from Vietnamese text
    fn remove_diacritics(&self, text: &str) -> String {
        text.chars()
            .map(|c| match c {
                'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => 'a',
                'â' | 'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'a',
                'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
                'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'e',
                'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
                'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => 'o',
                'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => 'o',
                'ơ' | 'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'o',
                'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => 'u',
                'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'u',
                'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
                'đ' => 'd',
                'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ' => 'A',
                'Â' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' => 'A',
                'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' => 'E',
                'Ê' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' => 'E',
                'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
                'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' => 'O',
                'Ô' | 'Ố' | 'Ồ' | 'Ổ' | 'Ỗ' | 'Ộ' => 'O',
                'Ơ' | 'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' => 'O',
                'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' => 'U',
                'Ư' | 'Ứ' | 'Ừ' | 'Ử' | 'Ữ' | 'Ự' => 'U',
                'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
                'Đ' => 'D',
                _ => c,
            })
            .collect()
    }

    /// Calculate edit distance between two strings
    fn edit_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let len1 = s1_chars.len();
        let len2 = s2_chars.len();

        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            dp[i][0] = i;
        }
        for j in 0..=len2 {
            dp[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1_chars[i - 1] == s2_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }

        dp[len1][len2]
    }
}

/// Spell checker statistics
#[derive(Debug, Clone)]
pub struct SpellCheckerStats {
    /// Number of Vietnamese words in dictionary
    pub vietnamese_word_count: usize,
    /// Number of English words in dictionary
    pub english_word_count: usize,
    /// Number of free marking patterns
    pub free_marking_pattern_count: usize,
    /// Total number of words
    pub total_word_count: usize,
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
