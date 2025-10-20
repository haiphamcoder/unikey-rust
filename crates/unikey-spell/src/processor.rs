//! Text processing with smart features

use crate::{SpellError, SpellResult, WordType, FreeMarkingMode};
use crate::{WordDetector, VietnameseDictionary, FreeMarkingProcessor};

/// Processing mode for text
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingMode {
    /// Normal processing
    Normal,
    /// Smart processing with context awareness
    Smart,
    /// Free marking mode
    FreeMarking,
}

/// Smart text processor
#[derive(Debug, Clone)]
pub struct SmartProcessor {
    /// Word detector
    detector: WordDetector,
    /// Vietnamese dictionary
    dictionary: VietnameseDictionary,
    /// Free marking processor
    free_marking: FreeMarkingProcessor,
    /// Current processing mode
    mode: ProcessingMode,
}

impl SmartProcessor {
    /// Create a new smart processor
    pub fn new() -> SpellResult<Self> {
        Ok(Self {
            detector: WordDetector::new()?,
            dictionary: VietnameseDictionary::with_common_words(),
            free_marking: FreeMarkingProcessor::new(),
            mode: ProcessingMode::Normal,
        })
    }

    /// Create with free marking enabled
    pub fn with_free_marking() -> SpellResult<Self> {
        Ok(Self {
            detector: WordDetector::new()?,
            dictionary: VietnameseDictionary::with_common_words(),
            free_marking: FreeMarkingProcessor::with_mode(FreeMarkingMode::Smart)
                .with_common_patterns(),
            mode: ProcessingMode::Smart,
        })
    }

    /// Set the processing mode
    pub fn set_mode(&mut self, mode: ProcessingMode) {
        self.mode = mode;
    }

    /// Get the current processing mode
    pub fn get_mode(&self) -> &ProcessingMode {
        &self.mode
    }

    /// Process text with smart features
    pub fn process_text(&mut self, text: &str) -> SpellResult<ProcessedText> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut processed_words = Vec::new();
        let mut word_types = Vec::new();
        let mut suggestions = Vec::new();

        for word in words {
            let word_type = self.detector.detect_word_type(word);
            word_types.push(word_type.clone());

            let processed_word = match word_type {
                WordType::Vietnamese => {
                    // Vietnamese word - check spelling
                    if self.dictionary.contains(word) {
                        word.to_string()
                    } else {
                        // Suggest corrections
                        let suggestion = self.suggest_correction(word);
                        let suggestion_text = suggestion.clone().unwrap_or_else(|| word.to_string());
                        suggestions.push(Suggestion {
                            original: word.to_string(),
                            suggestion: suggestion_text.clone(),
                            confidence: 0.8,
                        });
                        suggestion_text
                    }
                },
                WordType::VietnamesePlain => {
                    // Vietnamese word without diacritics - suggest with diacritics
                    let suggestion = self.suggest_with_diacritics(word);
                    let suggestion_text = suggestion.clone().unwrap_or_else(|| word.to_string());
                    suggestions.push(Suggestion {
                        original: word.to_string(),
                        suggestion: suggestion_text.clone(),
                        confidence: 0.9,
                    });
                    suggestion_text
                },
                WordType::English => {
                    // English word - apply free marking if enabled
                    if self.free_marking.should_activate(text, word_type) {
                        self.free_marking.process_text(word)?
                    } else {
                        word.to_string()
                    }
                },
                WordType::Mixed => {
                    // Mixed content - process each part
                    self.process_mixed_word(word)?
                },
                _ => {
                    // Other types - keep as is
                    word.to_string()
                }
            };

            processed_words.push(processed_word);
        }

        Ok(ProcessedText {
            original: text.to_string(),
            processed: processed_words.join(" "),
            word_types,
            suggestions,
            mode: self.mode.clone(),
        })
    }

    /// Suggest correction for a Vietnamese word
    fn suggest_correction(&self, word: &str) -> Option<String> {
        let word_lower = word.to_lowercase();
        
        // Simple edit distance-based suggestion
        let mut best_match = None;
        let mut best_distance = usize::MAX;

        for dict_word in self.dictionary.get_words() {
            let distance = self.edit_distance(&word_lower, dict_word);
            if distance < best_distance && distance <= 2 {
                best_distance = distance;
                best_match = Some(dict_word.clone());
            }
        }

        best_match
    }

    /// Suggest Vietnamese word with diacritics
    fn suggest_with_diacritics(&self, word: &str) -> Option<String> {
        let word_lower = word.to_lowercase();
        
        // Look for words that match without diacritics
        for dict_word in self.dictionary.get_words() {
            if self.remove_diacritics(dict_word) == word_lower {
                return Some(dict_word.clone());
            }
        }

        None
    }

    /// Process mixed word content
    fn process_mixed_word(&self, word: &str) -> SpellResult<String> {
        // For now, just return the word as is
        // TODO: Implement more sophisticated mixed content processing
        Ok(word.to_string())
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

    /// Get the word detector
    pub fn get_detector(&self) -> &WordDetector {
        &self.detector
    }

    /// Get the Vietnamese dictionary
    pub fn get_dictionary(&self) -> &VietnameseDictionary {
        &self.dictionary
    }

    /// Get the free marking processor
    pub fn get_free_marking(&self) -> &FreeMarkingProcessor {
        &self.free_marking
    }
}

/// Processed text result
#[derive(Debug, Clone)]
pub struct ProcessedText {
    /// Original text
    pub original: String,
    /// Processed text
    pub processed: String,
    /// Word types for each word
    pub word_types: Vec<WordType>,
    /// Suggestions for corrections
    pub suggestions: Vec<Suggestion>,
    /// Processing mode used
    pub mode: ProcessingMode,
}

/// Suggestion for word correction
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// Original word
    pub original: String,
    /// Suggested correction
    pub suggestion: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
}

impl Default for SmartProcessor {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
