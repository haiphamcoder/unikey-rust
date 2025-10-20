//! Vietnamese word detection and classification

use crate::{SpellError, SpellResult};
use regex::Regex;
use std::collections::HashSet;

/// Types of words that can be detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordType {
    /// Vietnamese word with diacritics
    Vietnamese,
    /// Vietnamese word without diacritics (plain)
    VietnamesePlain,
    /// English or other Latin script word
    English,
    /// Number
    Number,
    /// Punctuation or special characters
    Punctuation,
    /// Mixed content (Vietnamese + other)
    Mixed,
    /// Unknown or unrecognized
    Unknown,
}

/// Vietnamese word detector
#[derive(Debug, Clone)]
pub struct WordDetector {
    /// Vietnamese character patterns
    vietnamese_pattern: Regex,
    /// Vietnamese plain patterns (without diacritics)
    vietnamese_plain_pattern: Regex,
    /// English word patterns
    english_pattern: Regex,
    /// Number patterns
    number_pattern: Regex,
    /// Punctuation patterns
    punctuation_pattern: Regex,
    /// Common Vietnamese words dictionary
    vietnamese_words: HashSet<String>,
    /// Common English words dictionary
    english_words: HashSet<String>,
}

impl WordDetector {
    /// Create a new word detector
    pub fn new() -> SpellResult<Self> {
        let vietnamese_pattern = Regex::new(r"[\p{L}&&[\p{IsLatin}]]+")?;
        let vietnamese_plain_pattern = Regex::new(r"[a-zA-Z]+")?;
        let english_pattern = Regex::new(r"[a-zA-Z]+")?;
        let number_pattern = Regex::new(r"\d+")?;
        let punctuation_pattern = Regex::new(r"[^\p{L}\p{N}\s]+")?;

        let mut detector = Self {
            vietnamese_pattern,
            vietnamese_plain_pattern,
            english_pattern,
            number_pattern,
            punctuation_pattern,
            vietnamese_words: HashSet::new(),
            english_words: HashSet::new(),
        };

        // Initialize with common words
        detector.initialize_dictionaries();
        Ok(detector)
    }

    /// Initialize common word dictionaries
    fn initialize_dictionaries(&mut self) {
        // Common Vietnamese words
        let vietnamese_words = vec![
            "xin", "chào", "cảm", "ơn", "tạm", "biệt", "hẹn", "gặp", "lại",
            "tôi", "bạn", "anh", "chị", "em", "ông", "bà", "cô", "chú",
            "nhà", "trường", "học", "sinh", "viên", "giáo", "viên",
            "thành", "phố", "hà", "nội", "sài", "gòn", "việt", "nam",
            "quốc", "gia", "dân", "tộc", "văn", "hóa", "lịch", "sử",
            "kinh", "tế", "chính", "trị", "xã", "hội", "khoa", "học",
            "công", "nghệ", "thông", "tin", "máy", "tính", "điện", "thoại",
            "internet", "website", "email", "facebook", "youtube",
        ];

        for word in vietnamese_words {
            self.vietnamese_words.insert(word.to_lowercase());
        }

        // Common English words
        let english_words = vec![
            "hello", "world", "good", "morning", "afternoon", "evening",
            "night", "thank", "you", "please", "sorry", "excuse", "me",
            "yes", "no", "ok", "okay", "fine", "great", "good", "bad",
            "big", "small", "hot", "cold", "fast", "slow", "new", "old",
            "computer", "phone", "internet", "email", "website", "facebook",
            "youtube", "google", "microsoft", "apple", "amazon", "netflix",
        ];

        for word in english_words {
            self.english_words.insert(word.to_lowercase());
        }
    }

    /// Detect the type of a word
    pub fn detect_word_type(&self, word: &str) -> WordType {
        if word.is_empty() {
            return WordType::Unknown;
        }

        let word_lower = word.to_lowercase();

        // Check for numbers
        if self.number_pattern.is_match(word) {
            return WordType::Number;
        }

        // Check for punctuation
        if self.punctuation_pattern.is_match(word) {
            return WordType::Punctuation;
        }

        // Check for Vietnamese characters (with diacritics)
        if self.has_vietnamese_diacritics(word) {
            return WordType::Vietnamese;
        }

        // Check if it's a known Vietnamese word (without diacritics)
        if self.vietnamese_words.contains(&word_lower) {
            return WordType::VietnamesePlain;
        }

        // Check if it's a known English word
        if self.english_words.contains(&word_lower) {
            return WordType::English;
        }

        // Check for mixed content
        if self.has_mixed_content(word) {
            return WordType::Mixed;
        }

        // Default to unknown
        WordType::Unknown
    }

    /// Check if a word contains Vietnamese diacritics
    fn has_vietnamese_diacritics(&self, word: &str) -> bool {
        // Vietnamese diacritic characters
        let vietnamese_chars = [
            'á', 'à', 'ả', 'ã', 'ạ', 'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ',
            'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ', 'é', 'è', 'ẻ', 'ẽ', 'ẹ',
            'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ', 'í', 'ì', 'ỉ', 'ĩ', 'ị',
            'ó', 'ò', 'ỏ', 'õ', 'ọ', 'ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ',
            'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ', 'ú', 'ù', 'ủ', 'ũ', 'ụ',
            'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ',
            'đ', 'Đ', 'Á', 'À', 'Ả', 'Ã', 'Ạ', 'Ă', 'Ắ', 'Ằ', 'Ẳ',
            'Ẵ', 'Ặ', 'Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ', 'É', 'È', 'Ẻ',
            'Ẽ', 'Ẹ', 'Ê', 'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ', 'Í', 'Ì', 'Ỉ',
            'Ĩ', 'Ị', 'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ', 'Ô', 'Ố', 'Ồ', 'Ổ',
            'Ỗ', 'Ộ', 'Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ', 'Ú', 'Ù', 'Ủ',
            'Ũ', 'Ụ', 'Ư', 'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự', 'Ý', 'Ỳ', 'Ỷ',
            'Ỹ', 'Ỵ'
        ];

        word.chars().any(|c| vietnamese_chars.contains(&c))
    }

    /// Check if a word has mixed content (Vietnamese + other)
    fn has_mixed_content(&self, word: &str) -> bool {
        let has_vietnamese = self.has_vietnamese_diacritics(word);
        let has_english = self.english_pattern.is_match(word);
        let has_numbers = self.number_pattern.is_match(word);

        // Mixed if it has Vietnamese characters and other content
        has_vietnamese && (has_english || has_numbers)
    }

    /// Detect word type for a sequence of characters
    pub fn detect_sequence_type(&self, sequence: &str) -> WordType {
        let words: Vec<&str> = sequence.split_whitespace().collect();
        
        if words.is_empty() {
            return WordType::Unknown;
        }

        let mut vietnamese_count = 0;
        let mut english_count = 0;
        let mut other_count = 0;

        for word in &words {
            match self.detect_word_type(word) {
                WordType::Vietnamese | WordType::VietnamesePlain => vietnamese_count += 1,
                WordType::English => english_count += 1,
                _ => other_count += 1,
            }
        }

        let total_words = words.len();

        // If majority is Vietnamese
        if vietnamese_count > english_count && vietnamese_count > other_count {
            WordType::Vietnamese
        }
        // If majority is English
        else if english_count > vietnamese_count && english_count > other_count {
            WordType::English
        }
        // If mixed content
        else if vietnamese_count > 0 && english_count > 0 {
            WordType::Mixed
        }
        // Default to unknown
        else {
            WordType::Unknown
        }
    }

    /// Add a Vietnamese word to the dictionary
    pub fn add_vietnamese_word(&mut self, word: String) {
        self.vietnamese_words.insert(word.to_lowercase());
    }

    /// Add an English word to the dictionary
    pub fn add_english_word(&mut self, word: String) {
        self.english_words.insert(word.to_lowercase());
    }

    /// Get the number of Vietnamese words in dictionary
    pub fn vietnamese_word_count(&self) -> usize {
        self.vietnamese_words.len()
    }

    /// Get the number of English words in dictionary
    pub fn english_word_count(&self) -> usize {
        self.english_words.len()
    }
}

impl Default for WordDetector {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
