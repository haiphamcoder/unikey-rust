//! Core Vietnamese input engine

use crate::{KeyEvent, VnLexiName, CharType, KeyEventType, VowelSeq, CharMapper, is_word_break_char, is_vietnamese_char};
use std::collections::VecDeque;

/// Vietnamese input engine state
#[derive(Debug, Clone)]
pub struct EngineState {
    /// Current input buffer
    pub buffer: VecDeque<char>,
    /// Current vowel sequence
    pub vowel_seq: VowelSeq,
    /// Current tone
    pub tone: u8,
    /// Whether we're in Vietnamese input mode
    pub vietnamese_mode: bool,
    /// Current input method
    pub input_method: String,
    /// Output encoding
    pub output_encoding: String,
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            buffer: VecDeque::new(),
            vowel_seq: VowelSeq::Nil,
            tone: 0,
            vietnamese_mode: true,
            input_method: "Telex".to_string(),
            output_encoding: "Unicode".to_string(),
        }
    }
}

/// Vietnamese input engine
pub struct Engine {
    state: EngineState,
    engine_enabled: bool,
}

impl Engine {
    /// Create a new engine
    pub fn new() -> Self {
        Self {
            state: EngineState::default(),
            engine_enabled: true,
        }
    }

    /// Set input method
    pub fn set_input_method(&mut self, method: String) {
        self.state.input_method = method;
    }

    /// Get input method
    pub fn get_input_method(&self) -> &str {
        &self.state.input_method
    }

    /// Set output type
    pub fn set_output_type(&mut self, output_type: String) {
        self.state.output_encoding = output_type;
    }

    /// Get output type
    pub fn get_output_type(&self) -> &str {
        &self.state.output_encoding
    }

    /// Enable/disable engine
    pub fn enable_engine(&mut self, enabled: bool) {
        self.engine_enabled = enabled;
    }

    /// Check if engine is enabled
    pub fn is_engine_enabled(&self) -> bool {
        self.engine_enabled
    }

    /// Get current state
    pub fn get_state(&self) -> &EngineState {
        &self.state
    }

    /// Reset engine state
    pub fn reset(&mut self) {
        self.state.buffer.clear();
        self.state.vowel_seq = VowelSeq::Nil;
        self.state.tone = 0;
    }

    /// Process a key event
    pub fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        if !self.engine_enabled {
            return Ok(vec![]);
        }

        match event.char_type {
            CharType::Vn => {
                self.process_vietnamese_char(event)
            }
            CharType::WordBreak => {
                self.process_word_break(event)
            }
            CharType::NonVn => {
                self.process_non_vietnamese_char(event)
            }
            CharType::Reset => {
                self.reset();
                Ok(vec![])
            }
        }
    }

    /// Process Vietnamese character
    fn process_vietnamese_char(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        let ch = event.key_code as u8 as char;
        
        // Add character to buffer
        self.state.buffer.push_back(ch);
        
        // Try to complete a Vietnamese character from the current buffer
        if let Some(vietnamese_char) = self.try_complete_vietnamese_char() {
            // Clear buffer and return the completed character
            self.state.buffer.clear();
            self.state.vowel_seq = VowelSeq::Nil;
            self.state.tone = 0;
            return Ok(vietnamese_char);
        }
        
        // If buffer is getting too long, return the first character as-is
        if self.state.buffer.len() > 3 {
            let first_char = self.state.buffer.pop_front().unwrap();
            return Ok(vec![first_char as u8]);
        }
        
        // Return empty for now, character is being built
        Ok(vec![])
    }

    /// Process word break character
    fn process_word_break(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        let ch = event.key_code as u8 as char;
        
        // If we have a buffer, try to complete the Vietnamese character first
        if !self.state.buffer.is_empty() {
            if let Some(vietnamese_char) = self.try_complete_vietnamese_char() {
                self.state.buffer.clear();
                self.state.vowel_seq = VowelSeq::Nil;
                self.state.tone = 0;
                return Ok(vietnamese_char);
            }
        }
        
        // Return the word break character
        Ok(vec![ch as u8])
    }

    /// Process non-Vietnamese character
    fn process_non_vietnamese_char(&mut self, event: KeyEvent) -> Result<Vec<u8>, String> {
        let ch = event.key_code as u8 as char;
        
        // If we have a buffer, try to complete the Vietnamese character first
        if !self.state.buffer.is_empty() {
            if let Some(vietnamese_char) = self.try_complete_vietnamese_char() {
                self.state.buffer.clear();
                self.state.vowel_seq = VowelSeq::Nil;
                self.state.tone = 0;
                return Ok(vietnamese_char);
            }
        }
        
        // Return the non-Vietnamese character
        Ok(vec![ch as u8])
    }

    /// Try to complete a Vietnamese character from the current buffer
    fn try_complete_vietnamese_char(&mut self) -> Option<Vec<u8>> {
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
        
        // Try different input methods
        match self.state.input_method.as_str() {
            "Telex" => {
                if let Some(unicode) = CharMapper::telex_to_unicode(&buffer_str) {
                    return Some(unicode_to_utf8(unicode));
                }
            }
            "VNI" => {
                if let Some(unicode) = CharMapper::vni_to_unicode(&buffer_str) {
                    return Some(unicode_to_utf8(unicode));
                }
            }
            "VIQR" => {
                if let Some(unicode) = CharMapper::viqr_to_unicode(&buffer_str) {
                    return Some(unicode_to_utf8(unicode));
                }
            }
            _ => {}
        }
        
        None
    }

    /// Get current buffer as string
    pub fn get_buffer(&self) -> String {
        self.state.buffer.iter().collect()
    }

    /// Check if buffer is empty
    pub fn is_buffer_empty(&self) -> bool {
        self.state.buffer.is_empty()
    }

    /// Get buffer length
    pub fn buffer_length(&self) -> usize {
        self.state.buffer.len()
    }
}

/// Convert Unicode code point to UTF-8 bytes
fn unicode_to_utf8(unicode: u32) -> Vec<u8> {
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

/// Convert UTF-8 bytes to Unicode code point
fn utf8_to_unicode(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() {
        return None;
    }
    
    let first_byte = bytes[0];
    
    if first_byte <= 0x7F {
        // ASCII
        Some(first_byte as u32)
    } else if (first_byte & 0xE0) == 0xC0 && bytes.len() >= 2 {
        // 2-byte UTF-8
        Some(((first_byte & 0x1F) as u32) << 6 | ((bytes[1] & 0x3F) as u32))
    } else if (first_byte & 0xF0) == 0xE0 && bytes.len() >= 3 {
        // 3-byte UTF-8
        Some(((first_byte & 0x0F) as u32) << 12 | 
             ((bytes[1] & 0x3F) as u32) << 6 | 
             ((bytes[2] & 0x3F) as u32))
    } else if (first_byte & 0xF8) == 0xF0 && bytes.len() >= 4 {
        // 4-byte UTF-8
        Some(((first_byte & 0x07) as u32) << 18 | 
             ((bytes[1] & 0x3F) as u32) << 12 | 
             ((bytes[2] & 0x3F) as u32) << 6 | 
             ((bytes[3] & 0x3F) as u32))
    } else {
        None
    }
}

/// Alias for Engine
pub type UniKeyEngine = Engine;