//! Simplified TCVN3 encoding support

use std::collections::HashMap;

/// Simplified TCVN3 encoding utilities
pub struct Tcvn3SimpleEncoding {
    /// Unicode to TCVN3 mapping
    unicode_to_tcvn3: HashMap<u32, u8>,
    /// TCVN3 to Unicode mapping
    tcvn3_to_unicode: HashMap<u8, u32>,
}

impl Tcvn3SimpleEncoding {
    /// Create a new simplified TCVN3 encoding instance
    pub fn new() -> Self {
        let mut unicode_to_tcvn3 = HashMap::new();
        let mut tcvn3_to_unicode = HashMap::new();
        
        // Simplified TCVN3 character mappings
        // Only mapping Vietnamese-specific characters, ASCII stays the same
        let mappings = [
            // Vietnamese specific characters in TCVN3
            // A with tone marks
            (0x00C1, 0xC1), // Á
            (0x00C0, 0xC0), // À
            (0x1EA2, 0xC2), // Ả
            (0x00C3, 0xC3), // Ã
            (0x1EA0, 0xC4), // Ạ
            
            (0x00E1, 0xE1), // á
            (0x00E0, 0xE0), // à
            (0x1EA3, 0xE2), // ả
            (0x00E3, 0xE3), // ã
            (0x1EA1, 0xE4), // ạ
            
            // Ă with tone marks
            (0x0102, 0xC5), // Ă
            (0x1EAE, 0xC6), // Ằ
            (0x1EB0, 0xC7), // Ắ
            (0x1EB2, 0xC8), // Ẳ
            (0x1EB4, 0xC9), // Ẵ
            (0x1EB6, 0xCA), // Ặ
            
            (0x0103, 0xE5), // ă
            (0x1EAF, 0xE6), // ằ
            (0x1EB1, 0xE7), // ắ
            (0x1EB3, 0xE8), // ẳ
            (0x1EB5, 0xE9), // ẵ
            (0x1EB7, 0xEA), // ặ
            
            // Â with tone marks
            (0x00C2, 0xCB), // Â
            (0x1EA0, 0xCC), // Ầ
            (0x1EA2, 0xCD), // Ấ
            (0x1EA4, 0xCE), // Ẩ
            (0x1EA6, 0xCF), // Ẫ
            (0x1EA8, 0xD0), // Ậ
            
            (0x00E2, 0xEB), // â
            (0x1EA1, 0xEC), // ầ
            (0x1EA3, 0xED), // ấ
            (0x1EA5, 0xEE), // ẩ
            (0x1EA7, 0xEF), // ẫ
            (0x1EA9, 0xF0), // ậ
            
            // E with tone marks
            (0x00C8, 0xD1), // È
            (0x00C9, 0xD2), // É
            (0x1EBA, 0xD3), // Ẻ
            (0x1EBC, 0xD4), // Ẽ
            (0x1EB8, 0xD5), // Ẹ
            
            (0x00E8, 0xF1), // è
            (0x00E9, 0xF2), // é
            (0x1EBB, 0xF3), // ẻ
            (0x1EBD, 0xF4), // ẽ
            (0x1EB9, 0xF5), // ẹ
            
            // Ê with tone marks
            (0x00CA, 0xD6), // Ê
            (0x1EC0, 0xD7), // Ề
            (0x1EC2, 0xD8), // Ế
            (0x1EC4, 0xD9), // Ể
            (0x1EC6, 0xDA), // Ễ
            (0x1EC8, 0xDB), // Ệ
            
            (0x00EA, 0xF6), // ê
            (0x1EC1, 0xF7), // ề
            (0x1EC3, 0xF8), // ế
            (0x1EC5, 0xF9), // ể
            (0x1EC7, 0xFA), // ễ
            (0x1EC9, 0xFB), // ệ
            
            // I with tone marks
            (0x00CC, 0xCC), // Ì
            (0x00CD, 0xCD), // Í
            (0x1EC8, 0xCE), // Ỉ
            (0x0128, 0xCF), // Ĩ
            (0x1ECA, 0xD0), // Ị
            
            (0x00EC, 0xEC), // ì
            (0x00ED, 0xED), // í
            (0x1EC9, 0xEE), // ỉ
            (0x0129, 0xEF), // ĩ
            (0x1ECB, 0xF0), // ị
            
            // O with tone marks
            (0x00D2, 0xD2), // Ò
            (0x00D3, 0xD3), // Ó
            (0x1ECE, 0xD4), // Ỏ
            (0x00D5, 0xD5), // Õ
            (0x1ECC, 0xD6), // Ọ
            
            (0x00F2, 0xF2), // ò
            (0x00F3, 0xF3), // ó
            (0x1ECF, 0xF4), // ỏ
            (0x00F5, 0xF5), // õ
            (0x1ECD, 0xF6), // ọ
            
            // Ô with tone marks
            (0x00D4, 0xD7), // Ô
            (0x1ED0, 0xD8), // Ồ
            (0x1ED2, 0xD9), // Ố
            (0x1ED4, 0xDA), // Ổ
            (0x1ED6, 0xDB), // Ỗ
            (0x1ED8, 0xDC), // Ộ
            
            (0x00F4, 0xF7), // ô
            (0x1ED1, 0xF8), // ồ
            (0x1ED3, 0xF9), // ố
            (0x1ED5, 0xFA), // ổ
            (0x1ED7, 0xFB), // ỗ
            (0x1ED9, 0xFC), // ộ
            
            // Ơ with tone marks
            (0x01A0, 0xDD), // Ơ
            (0x1EDA, 0xDE), // Ớ
            (0x1EDC, 0xDF), // Ờ
            (0x1EDE, 0xE0), // Ở
            (0x1EE0, 0xE1), // Ỡ
            (0x1EE2, 0xE2), // Ợ
            
            (0x01A1, 0xFD), // ơ
            (0x1EDB, 0xFE), // ớ
            (0x1EDD, 0xFF), // ờ
            (0x1EDF, 0x80), // ở
            (0x1EE1, 0x81), // ỡ
            (0x1EE3, 0x82), // ợ
            
            // U with tone marks
            (0x00D9, 0x83), // Ù
            (0x00DA, 0x84), // Ú
            (0x1EE6, 0x85), // Ủ
            (0x0168, 0x86), // Ũ
            (0x1EE4, 0x87), // Ụ
            
            (0x00F9, 0x88), // ù
            (0x00FA, 0x89), // ú
            (0x1EE7, 0x8A), // ủ
            (0x0169, 0x8B), // ũ
            (0x1EE5, 0x8C), // ụ
            
            // Ư with tone marks
            (0x01AF, 0x8D), // Ư
            (0x1EE8, 0x8E), // Ứ
            (0x1EEA, 0x8F), // Ừ
            (0x1EEC, 0x90), // Ử
            (0x1EEE, 0x91), // Ữ
            (0x1EF0, 0x92), // Ự
            
            (0x01B0, 0x93), // ư
            (0x1EE9, 0x94), // ứ
            (0x1EEB, 0x95), // ừ
            (0x1EED, 0x96), // ử
            (0x1EEF, 0x97), // ữ
            (0x1EF1, 0x98), // ự
            
            // Y with tone marks
            (0x1EF2, 0x99), // Ỳ
            (0x1EF4, 0x9A), // Ý
            (0x1EF6, 0x9B), // Ỷ
            (0x1EF8, 0x9C), // Ỹ
            (0x1EFA, 0x9D), // Ỵ
            
            (0x1EF3, 0x9E), // ỳ
            (0x1EF5, 0x9F), // ý
            (0x1EF7, 0xA0), // ỷ
            (0x1EF9, 0xA1), // ỹ
            (0x1EFB, 0xA2), // ỵ
            
            // Special characters
            (0x0110, 0xA3), // Đ
            (0x0111, 0xA4), // đ
        ];
        
        for (unicode, tcvn3) in mappings.iter() {
            unicode_to_tcvn3.insert(*unicode, *tcvn3);
            tcvn3_to_unicode.insert(*tcvn3, *unicode);
        }
        
        Self {
            unicode_to_tcvn3,
            tcvn3_to_unicode,
        }
    }
    
    /// Convert Unicode string to TCVN3 bytes
    pub fn to_tcvn3(&self, input: &str) -> Vec<u8> {
        let mut result = Vec::new();
        
        for ch in input.chars() {
            let unicode = ch as u32;
            if let Some(&tcvn3) = self.unicode_to_tcvn3.get(&unicode) {
                result.push(tcvn3);
            } else {
                // For unmapped characters, try to convert to ASCII
                if unicode <= 0x7F {
                    result.push(unicode as u8);
                } else {
                    // Replace with question mark for unmapped characters
                    result.push(0x3F); // '?'
                }
            }
        }
        
        result
    }
    
    /// Convert TCVN3 bytes to Unicode string
    pub fn from_tcvn3(&self, input: &[u8]) -> String {
        let mut result = String::new();
        
        for &byte in input {
            if let Some(&unicode) = self.tcvn3_to_unicode.get(&byte) {
                if let Some(ch) = std::char::from_u32(unicode) {
                    result.push(ch);
                } else {
                    result.push('?');
                }
            } else if byte <= 0x7F {
                // ASCII characters
                result.push(byte as char);
            } else {
                result.push('?');
            }
        }
        
        result
    }
    
    /// Check if a byte is valid TCVN3
    pub fn is_valid_tcvn3(&self, byte: u8) -> bool {
        self.tcvn3_to_unicode.contains_key(&byte) || byte <= 0x7F
    }
    
    /// Get all supported Unicode characters
    pub fn get_supported_unicode_chars(&self) -> Vec<u32> {
        self.unicode_to_tcvn3.keys().copied().collect()
    }
}

impl Default for Tcvn3SimpleEncoding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tcvn3_simple_conversion() {
        let encoding = Tcvn3SimpleEncoding::new();
        
        // Test basic ASCII
        let ascii = "Hello World";
        let tcvn3 = encoding.to_tcvn3(ascii);
        let back = encoding.from_tcvn3(&tcvn3);
        assert_eq!(ascii, back);
        
        // Test Vietnamese characters
        let vietnamese = "Xin chào thế giới";
        let tcvn3 = encoding.to_tcvn3(vietnamese);
        let back = encoding.from_tcvn3(&tcvn3);
        assert_eq!(vietnamese, back);
    }
    
    #[test]
    fn test_tcvn3_simple_specific_chars() {
        let encoding = Tcvn3SimpleEncoding::new();
        
        // Test specific Vietnamese characters
        let test_cases = vec![
            ("á", vec![0xE1]),
            ("à", vec![0xE0]),
            ("ả", vec![0xE2]),
            ("ã", vec![0xE3]),
            ("ạ", vec![0xE4]),
            ("ă", vec![0xE5]),
            ("â", vec![0xEB]),
            ("đ", vec![0xA4]),
            ("Đ", vec![0xA3]),
        ];
        
        for (unicode_str, expected_tcvn3) in test_cases {
            let tcvn3 = encoding.to_tcvn3(unicode_str);
            assert_eq!(tcvn3, expected_tcvn3);
            
            let back = encoding.from_tcvn3(&tcvn3);
            assert_eq!(back, unicode_str);
        }
    }
    
    #[test]
    fn test_tcvn3_simple_validation() {
        let encoding = Tcvn3SimpleEncoding::new();
        
        // Test valid TCVN3 bytes
        assert!(encoding.is_valid_tcvn3(0x41)); // 'A'
        assert!(encoding.is_valid_tcvn3(0xE1)); // 'á'
        assert!(encoding.is_valid_tcvn3(0xA4)); // 'đ'
        
        // Test invalid TCVN3 bytes
        assert!(!encoding.is_valid_tcvn3(0x80));
        assert!(!encoding.is_valid_tcvn3(0xFF));
    }
}
