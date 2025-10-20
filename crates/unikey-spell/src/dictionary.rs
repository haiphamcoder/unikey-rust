//! Vietnamese dictionary management

use crate::{SpellError, SpellResult};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Vietnamese dictionary for spell checking
#[derive(Debug, Clone)]
pub struct VietnameseDictionary {
    /// Set of Vietnamese words
    words: HashSet<String>,
    /// Set of Vietnamese word patterns
    patterns: HashSet<String>,
    /// Dictionary file path
    file_path: Option<String>,
}

impl VietnameseDictionary {
    /// Create a new empty dictionary
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
            patterns: HashSet::new(),
            file_path: None,
        }
    }

    /// Create a dictionary with common Vietnamese words
    pub fn with_common_words() -> Self {
        let mut dict = Self::new();
        dict.load_common_words();
        dict
    }

    /// Load dictionary from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> SpellResult<Self> {
        let mut dict = Self::new();
        dict.file_path = Some(path.as_ref().to_string_lossy().to_string());
        dict.load_from_file()?;
        Ok(dict)
    }

    /// Load common Vietnamese words
    fn load_common_words(&mut self) {
        let common_words = vec![
            // Basic words
            "xin", "chào", "cảm", "ơn", "tạm", "biệt", "hẹn", "gặp", "lại",
            "tôi", "bạn", "anh", "chị", "em", "ông", "bà", "cô", "chú",
            
            // Family
            "bố", "mẹ", "cha", "má", "con", "cháu", "chắt", "cụ", "kỵ",
            "anh", "chị", "em", "trai", "gái", "ruột", "họ", "ngoại",
            
            // Places
            "nhà", "trường", "học", "sinh", "viên", "giáo", "viên", "thầy", "cô",
            "thành", "phố", "hà", "nội", "sài", "gòn", "việt", "nam", "quốc", "gia",
            "tỉnh", "huyện", "xã", "phường", "đường", "phố", "ngõ", "hẻm",
            
            // Time
            "năm", "tháng", "ngày", "giờ", "phút", "giây", "sáng", "trưa", "chiều", "tối",
            "hôm", "nay", "qua", "mai", "tuần", "tháng", "năm", "thế", "kỷ",
            
            // Colors
            "đỏ", "xanh", "vàng", "trắng", "đen", "nâu", "hồng", "tím", "cam", "xám",
            
            // Numbers
            "một", "hai", "ba", "bốn", "năm", "sáu", "bảy", "tám", "chín", "mười",
            "mười", "một", "mười", "hai", "hai", "mươi", "ba", "mươi", "bốn", "mươi",
            "năm", "mươi", "sáu", "mươi", "bảy", "mươi", "tám", "mươi", "chín", "mươi",
            "trăm", "nghìn", "triệu", "tỷ",
            
            // Food
            "cơm", "phở", "bún", "miến", "cháo", "canh", "thịt", "cá", "tôm", "cua",
            "rau", "củ", "quả", "trái", "cây", "hoa", "lá", "cành", "rễ", "thân",
            
            // Technology
            "máy", "tính", "điện", "thoại", "internet", "website", "email", "facebook",
            "youtube", "google", "microsoft", "apple", "amazon", "netflix", "wifi",
            "bluetooth", "usb", "camera", "microphone", "speaker", "headphone",
            
            // Common verbs
            "làm", "đi", "đến", "về", "ở", "ngồi", "đứng", "nằm", "chạy", "đi", "bộ",
            "ăn", "uống", "ngủ", "thức", "dậy", "tắm", "rửa", "lau", "quét", "dọn",
            "học", "dạy", "đọc", "viết", "nói", "nghe", "nhìn", "thấy", "biết", "hiểu",
            "yêu", "ghét", "thích", "không", "thích", "muốn", "cần", "phải", "nên",
            
            // Common adjectives
            "tốt", "xấu", "đẹp", "xấu", "lớn", "nhỏ", "cao", "thấp", "dài", "ngắn",
            "rộng", "hẹp", "dày", "mỏng", "nặng", "nhẹ", "nhanh", "chậm", "mới", "cũ",
            "sạch", "bẩn", "nóng", "lạnh", "ấm", "mát", "khô", "ướt", "sáng", "tối",
            
            // Common nouns
            "người", "đàn", "ông", "bà", "cô", "chú", "anh", "chị", "em", "bạn",
            "đồng", "nghiệp", "học", "sinh", "giáo", "viên", "bác", "sĩ", "kỹ", "sư",
            "công", "nhân", "nông", "dân", "thương", "gia", "doanh", "nhân", "nghệ", "sĩ",
        ];

        for word in common_words {
            self.words.insert(word.to_lowercase());
        }

        // Add common patterns
        let patterns = vec![
            "xin chào", "cảm ơn", "tạm biệt", "hẹn gặp lại",
            "tôi là", "bạn là", "anh là", "chị là", "em là",
            "nhà tôi", "trường học", "thành phố", "quốc gia",
            "hôm nay", "ngày mai", "tuần sau", "tháng sau", "năm sau",
            "máy tính", "điện thoại", "internet", "website", "email",
        ];

        for pattern in patterns {
            self.patterns.insert(pattern.to_lowercase());
        }
    }

    /// Load dictionary from file
    fn load_from_file(&mut self) -> SpellResult<()> {
        if let Some(ref path) = self.file_path {
            if Path::new(path).exists() {
                let content = fs::read_to_string(path)?;
                for line in content.lines() {
                    let word = line.trim().to_lowercase();
                    if !word.is_empty() {
                        self.words.insert(word);
                    }
                }
            }
        }
        Ok(())
    }

    /// Save dictionary to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> SpellResult<()> {
        let mut content = String::new();
        for word in &self.words {
            content.push_str(word);
            content.push('\n');
        }
        fs::write(path, content)?;
        Ok(())
    }

    /// Check if a word exists in the dictionary
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }

    /// Check if a pattern exists in the dictionary
    pub fn contains_pattern(&self, pattern: &str) -> bool {
        self.patterns.contains(&pattern.to_lowercase())
    }

    /// Add a word to the dictionary
    pub fn add_word(&mut self, word: String) {
        self.words.insert(word.to_lowercase());
    }

    /// Add a pattern to the dictionary
    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.insert(pattern.to_lowercase());
    }

    /// Remove a word from the dictionary
    pub fn remove_word(&mut self, word: &str) -> bool {
        self.words.remove(&word.to_lowercase())
    }

    /// Get all words in the dictionary
    pub fn get_words(&self) -> &HashSet<String> {
        &self.words
    }

    /// Get all patterns in the dictionary
    pub fn get_patterns(&self) -> &HashSet<String> {
        &self.patterns
    }

    /// Get the number of words in the dictionary
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Get the number of patterns in the dictionary
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Clear the dictionary
    pub fn clear(&mut self) {
        self.words.clear();
        self.patterns.clear();
    }

    /// Merge another dictionary into this one
    pub fn merge(&mut self, other: &VietnameseDictionary) {
        for word in &other.words {
            self.words.insert(word.clone());
        }
        for pattern in &other.patterns {
            self.patterns.insert(pattern.clone());
        }
    }
}

impl Default for VietnameseDictionary {
    fn default() -> Self {
        Self::with_common_words()
    }
}
