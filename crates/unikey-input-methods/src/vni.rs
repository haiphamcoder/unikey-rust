//! VNI input method implementation

use crate::traits::InputMethod;
use unikey_core::{KeyEvent, CharType, KeyEventType, CharMapper, is_word_break_char};
use std::collections::VecDeque;

/// VNI input method state
#[derive(Debug, Clone)]
pub struct VniState {
    /// Current input buffer
    buffer: VecDeque<char>,
    /// Whether we're in Vietnamese input mode
    vietnamese_mode: bool,
    /// Current tone (0-5)
    tone: u8,
    /// Whether we're building a character
    building: bool,
}

impl Default for VniState {
    fn default() -> Self {
        Self {
            buffer: VecDeque::new(),
            vietnamese_mode: true,
            tone: 0,
            building: false,
        }
    }
}

/// VNI input method
pub struct VniMethod {
    state: VniState,
}

impl VniMethod {
    /// Create a new VNI method
    pub fn new() -> Self {
        Self {
            state: VniState::default(),
        }
    }
    
    /// Process a single character input
    fn process_char(&mut self, ch: char) -> Result<Vec<u8>, String> {
        // Add character to buffer
        self.state.buffer.push_back(ch);
        self.state.building = true;
        
        // Try to complete a Vietnamese character
        if let Some(vietnamese_char) = self.try_complete_character() {
            self.state.buffer.clear();
            self.state.tone = 0;
            self.state.building = false;
            return Ok(vietnamese_char);
        }
        
        // If buffer is getting too long, return the first character as-is
        if self.state.buffer.len() > 4 {
            let first_char = self.state.buffer.pop_front().unwrap();
            return Ok(vec![first_char as u8]);
        }
        
        // Return empty for now, character is being built
        Ok(vec![])
    }
    
    /// Try to complete a Vietnamese character from the current buffer
    fn try_complete_character(&self) -> Option<Vec<u8>> {
        let buffer_str: String = self.state.buffer.iter().collect();
        
        // Only try to complete if buffer has more than 1 character
        // or if it's a single character that's not a basic ASCII letter
        if buffer_str.len() == 1 {
            let ch = buffer_str.chars().next().unwrap();
            // Only complete single characters that are Vietnamese-specific
            if ch.is_ascii_alphabetic() {
                return None; // Let it build up more characters
            }
        }
        
        // Try to find a mapping for the current buffer
        if let Some(unicode) = CharMapper::vni_to_unicode(&buffer_str) {
            return Some(self.unicode_to_utf8(unicode));
        }
        
        None
    }
    
    /// Convert Unicode code point to UTF-8 bytes
    fn unicode_to_utf8(&self, unicode: u32) -> Vec<u8> {
        let mut result = Vec::new();
        
        if unicode <= 0x7F {
            // ASCII
            result.push(unicode as u8);
        } else if unicode <= 0x7FF {
            // 2-byte UTF-8
            result.push(0xC0 | ((unicode >> 6) as u8));
            result.push(0x80 | ((unicode & 0x3F) as u8));
        } else if unicode <= 0xFFFF {
            // 3-byte UTF-8
            result.push(0xE0 | ((unicode >> 12) as u8));
            result.push(0x80 | (((unicode >> 6) & 0x3F) as u8));
            result.push(0x80 | ((unicode & 0x3F) as u8));
        } else {
            // 4-byte UTF-8
            result.push(0xF0 | ((unicode >> 18) as u8));
            result.push(0x80 | (((unicode >> 12) & 0x3F) as u8));
            result.push(0x80 | (((unicode >> 6) & 0x3F) as u8));
            result.push(0x80 | ((unicode & 0x3F) as u8));
        }
        
        result
    }
    
    /// Process word break
    fn process_word_break(&mut self, ch: char) -> Result<Vec<u8>, String> {
        // If we have a buffer, try to complete the Vietnamese character first
        if !self.state.buffer.is_empty() {
            if let Some(vietnamese_char) = self.try_complete_character() {
                self.state.buffer.clear();
                self.state.tone = 0;
                self.state.building = false;
                return Ok(vietnamese_char);
            }
        }
        
        // Return the word break character
        Ok(vec![ch as u8])
    }
    
    /// Process non-Vietnamese character
    fn process_non_vietnamese(&mut self, ch: char) -> Result<Vec<u8>, String> {
        // If we have a buffer, try to complete the Vietnamese character first
        if !self.state.buffer.is_empty() {
            if let Some(vietnamese_char) = self.try_complete_character() {
                self.state.buffer.clear();
                self.state.tone = 0;
                self.state.building = false;
                return Ok(vietnamese_char);
            }
        }
        
        // Return the non-Vietnamese character
        Ok(vec![ch as u8])
    }
    
    /// Get current buffer as string
    pub fn get_buffer(&self) -> String {
        self.state.buffer.iter().collect()
    }
    
    /// Check if we're currently building a character
    pub fn is_building(&self) -> bool {
        self.state.building
    }
    
    /// Get current tone
    pub fn get_tone(&self) -> u8 {
        self.state.tone
    }
}

impl InputMethod for VniMethod {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        if !self.state.vietnamese_mode {
            return Ok(vec![event.key_code as u8]);
        }
        
        let ch = event.key_code as u8 as char;
        
        match event.char_type {
            CharType::Vn => {
                self.process_char(ch)
            }
            CharType::WordBreak => {
                self.process_word_break(ch)
            }
            CharType::NonVn => {
                self.process_non_vietnamese(ch)
            }
            CharType::Reset => {
                self.reset();
                Ok(vec![])
            }
        }
    }
    
    fn reset(&mut self) {
        self.state.buffer.clear();
        self.state.tone = 0;
        self.state.building = false;
    }
    
    fn name(&self) -> &str {
        "VNI"
    }
}

impl Default for VniMethod {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unikey_core::{KeyEvent, KeyEventType, CharType, VnLexiName};
    
    #[test]
    fn test_vni_basic_characters() {
        let mut vni = VniMethod::new();
        
        // Test 'a' -> wait
        let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
        let result = vni.process_key(key_a).unwrap();
        assert!(result.is_empty());
        assert_eq!(vni.get_buffer(), "a");
        
        // Test 'a6' -> 'â'
        let key_6 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 54);
        let result = vni.process_key(key_6).unwrap();
        assert_eq!(String::from_utf8_lossy(&result), "â");
        assert!(vni.get_buffer().is_empty());
    }
    
    #[test]
    fn test_vni_diacritics() {
        let mut vni = VniMethod::new();
        
        // Test 'a8' -> 'ă'
        let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
        let _ = vni.process_key(key_a);
        let key_8 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 56);
        let result = vni.process_key(key_8).unwrap();
        assert_eq!(String::from_utf8_lossy(&result), "ă");
    }
    
    #[test]
    fn test_vni_tone_marks() {
        let mut vni = VniMethod::new();
        
        // Test 'a1' -> 'á'
        let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
        let _ = vni.process_key(key_a);
        let key_1 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 49);
        let result = vni.process_key(key_1).unwrap();
        assert_eq!(String::from_utf8_lossy(&result), "á");
    }
    
    #[test]
    fn test_vni_word_break() {
        let mut vni = VniMethod::new();
        
        // Test 'a' + space -> 'a'
        let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
        let _ = vni.process_key(key_a);
        let key_space = KeyEvent::new(KeyEventType::Normal, CharType::WordBreak, VnLexiName::A, 32);
        let result = vni.process_key(key_space).unwrap();
        assert_eq!(String::from_utf8_lossy(&result), " ");
    }
}