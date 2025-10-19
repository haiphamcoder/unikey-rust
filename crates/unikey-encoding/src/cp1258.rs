//! Windows CP1258 encoding support

use std::collections::HashMap;

/// CP1258 encoding utilities
pub struct Cp1258Encoding {
    /// Unicode to CP1258 mapping
    unicode_to_cp1258: HashMap<u32, u8>,
    /// CP1258 to Unicode mapping
    cp1258_to_unicode: HashMap<u8, u32>,
}

impl Cp1258Encoding {
    /// Create a new CP1258 encoding instance
    pub fn new() -> Self {
        let mut unicode_to_cp1258 = HashMap::new();
        let mut cp1258_to_unicode = HashMap::new();
        
        // CP1258 character mappings
        // Based on Windows Code Page 1258 (Vietnamese)
        let mappings = [
            // Basic Latin (0x00-0x7F) - same as ASCII
            (0x0041, 0x41), // A
            (0x0042, 0x42), // B
            (0x0043, 0x43), // C
            (0x0044, 0x44), // D
            (0x0045, 0x45), // E
            (0x0046, 0x46), // F
            (0x0047, 0x47), // G
            (0x0048, 0x48), // H
            (0x0049, 0x49), // I
            (0x004A, 0x4A), // J
            (0x004B, 0x4B), // K
            (0x004C, 0x4C), // L
            (0x004D, 0x4D), // M
            (0x004E, 0x4E), // N
            (0x004F, 0x4F), // O
            (0x0050, 0x50), // P
            (0x0051, 0x51), // Q
            (0x0052, 0x52), // R
            (0x0053, 0x53), // S
            (0x0054, 0x54), // T
            (0x0055, 0x55), // U
            (0x0056, 0x56), // V
            (0x0057, 0x57), // W
            (0x0058, 0x58), // X
            (0x0059, 0x59), // Y
            (0x005A, 0x5A), // Z
            
            (0x0061, 0x61), // a
            (0x0062, 0x62), // b
            (0x0063, 0x63), // c
            (0x0064, 0x64), // d
            (0x0065, 0x65), // e
            (0x0066, 0x66), // f
            (0x0067, 0x67), // g
            (0x0068, 0x68), // h
            (0x0069, 0x69), // i
            (0x006A, 0x6A), // j
            (0x006B, 0x6B), // k
            (0x006C, 0x6C), // l
            (0x006D, 0x6D), // m
            (0x006E, 0x6E), // n
            (0x006F, 0x6F), // o
            (0x0070, 0x70), // p
            (0x0071, 0x71), // q
            (0x0072, 0x72), // r
            (0x0073, 0x73), // s
            (0x0074, 0x74), // t
            (0x0075, 0x75), // u
            (0x0076, 0x76), // v
            (0x0077, 0x77), // w
            (0x0078, 0x78), // x
            (0x0079, 0x79), // y
            (0x007A, 0x7A), // z
            
            // Vietnamese specific characters in CP1258
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
            (0x00CC, 0xDC), // Ì
            (0x00CD, 0xDD), // Í
            (0x1EC8, 0xDE), // Ỉ
            (0x0128, 0xDF), // Ĩ
            (0x1ECA, 0xE0), // Ị
            
            (0x00EC, 0xFC), // ì
            (0x00ED, 0xFD), // í
            (0x1EC9, 0xFE), // ỉ
            (0x0129, 0xFF), // ĩ
            (0x1ECB, 0x00), // ị
            
            // O with tone marks
            (0x00D2, 0x01), // Ò
            (0x00D3, 0x02), // Ó
            (0x1ECE, 0x03), // Ỏ
            (0x00D5, 0x04), // Õ
            (0x1ECC, 0x05), // Ọ
            
            (0x00F2, 0x06), // ò
            (0x00F3, 0x07), // ó
            (0x1ECF, 0x08), // ỏ
            (0x00F5, 0x09), // õ
            (0x1ECD, 0x0A), // ọ
            
            // Ô with tone marks
            (0x00D4, 0x0B), // Ô
            (0x1ED0, 0x0C), // Ồ
            (0x1ED2, 0x0D), // Ố
            (0x1ED4, 0x0E), // Ổ
            (0x1ED6, 0x0F), // Ỗ
            (0x1ED8, 0x10), // Ộ
            
            (0x00F4, 0x11), // ô
            (0x1ED1, 0x12), // ồ
            (0x1ED3, 0x13), // ố
            (0x1ED5, 0x14), // ổ
            (0x1ED7, 0x15), // ỗ
            (0x1ED9, 0x16), // ộ
            
            // Ơ with tone marks
            (0x01A0, 0x17), // Ơ
            (0x1EDA, 0x18), // Ớ
            (0x1EDC, 0x19), // Ờ
            (0x1EDE, 0x1A), // Ở
            (0x1EE0, 0x1B), // Ỡ
            (0x1EE2, 0x1C), // Ợ
            
            (0x01A1, 0x1D), // ơ
            (0x1EDB, 0x1E), // ớ
            (0x1EDD, 0x1F), // ờ
            (0x1EDF, 0x20), // ở
            (0x1EE1, 0x21), // ỡ
            (0x1EE3, 0x22), // ợ
            
            // U with tone marks
            (0x00D9, 0x23), // Ù
            (0x00DA, 0x24), // Ú
            (0x1EE6, 0x25), // Ủ
            (0x0168, 0x26), // Ũ
            (0x1EE4, 0x27), // Ụ
            
            (0x00F9, 0x28), // ù
            (0x00FA, 0x29), // ú
            (0x1EE7, 0x2A), // ủ
            (0x0169, 0x2B), // ũ
            (0x1EE5, 0x2C), // ụ
            
            // Ư with tone marks
            (0x01AF, 0x2D), // Ư
            (0x1EE8, 0x2E), // Ứ
            (0x1EEA, 0x2F), // Ừ
            (0x1EEC, 0x30), // Ử
            (0x1EEE, 0x31), // Ữ
            (0x1EF0, 0x32), // Ự
            
            (0x01B0, 0x33), // ư
            (0x1EE9, 0x34), // ứ
            (0x1EEB, 0x35), // ừ
            (0x1EED, 0x36), // ử
            (0x1EEF, 0x37), // ữ
            (0x1EF1, 0x38), // ự
            
            // Y with tone marks
            (0x1EF2, 0x39), // Ỳ
            (0x1EF4, 0x3A), // Ý
            (0x1EF6, 0x3B), // Ỷ
            (0x1EF8, 0x3C), // Ỹ
            (0x1EFA, 0x3D), // Ỵ
            
            (0x1EF3, 0x3E), // ỳ
            (0x1EF5, 0x3F), // ý
            (0x1EF7, 0x40), // ỷ
            (0x1EF9, 0x5B), // ỹ
            (0x1EFB, 0x5C), // ỵ
            
            // Special characters
            (0x0110, 0x5D), // Đ
            (0x0111, 0x5E), // đ
        ];
        
        for (unicode, cp1258) in mappings.iter() {
            unicode_to_cp1258.insert(*unicode, *cp1258);
            cp1258_to_unicode.insert(*cp1258, *unicode);
        }
        
        Self {
            unicode_to_cp1258,
            cp1258_to_unicode,
        }
    }
    
    /// Convert Unicode string to CP1258 bytes
    pub fn to_cp1258(&self, input: &str) -> Vec<u8> {
        let mut result = Vec::new();
        
        for ch in input.chars() {
            let unicode = ch as u32;
            if let Some(&cp1258) = self.unicode_to_cp1258.get(&unicode) {
                result.push(cp1258);
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
    
    /// Convert CP1258 bytes to Unicode string
    pub fn from_cp1258(&self, input: &[u8]) -> String {
        let mut result = String::new();
        
        for &byte in input {
            if let Some(&unicode) = self.cp1258_to_unicode.get(&byte) {
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
    
    /// Check if a byte is valid CP1258
    pub fn is_valid_cp1258(&self, byte: u8) -> bool {
        self.cp1258_to_unicode.contains_key(&byte) || byte <= 0x7F
    }
    
    /// Get all supported Unicode characters
    pub fn get_supported_unicode_chars(&self) -> Vec<u32> {
        self.unicode_to_cp1258.keys().copied().collect()
    }
}

impl Default for Cp1258Encoding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cp1258_conversion() {
        let encoding = Cp1258Encoding::new();
        
        // Test basic ASCII
        let ascii = "Hello World";
        let cp1258 = encoding.to_cp1258(ascii);
        let back = encoding.from_cp1258(&cp1258);
        assert_eq!(ascii, back);
        
        // Test Vietnamese characters
        let vietnamese = "Xin chào thế giới";
        let cp1258 = encoding.to_cp1258(vietnamese);
        let back = encoding.from_cp1258(&cp1258);
        assert_eq!(vietnamese, back);
    }
    
    #[test]
    fn test_cp1258_specific_chars() {
        let encoding = Cp1258Encoding::new();
        
        // Test specific Vietnamese characters
        let test_cases = vec![
            ("á", vec![0xE1]),
            ("à", vec![0xE0]),
            ("ả", vec![0xE2]),
            ("ã", vec![0xE3]),
            ("ạ", vec![0xE4]),
            ("ă", vec![0xE5]),
            ("â", vec![0xEB]),
            ("đ", vec![0x5E]),
            ("Đ", vec![0x5D]),
        ];
        
        for (unicode_str, expected_cp1258) in test_cases {
            let cp1258 = encoding.to_cp1258(unicode_str);
            assert_eq!(cp1258, expected_cp1258);
            
            let back = encoding.from_cp1258(&cp1258);
            assert_eq!(back, unicode_str);
        }
    }
    
    #[test]
    fn test_cp1258_validation() {
        let encoding = Cp1258Encoding::new();
        
        // Test valid CP1258 bytes
        assert!(encoding.is_valid_cp1258(0x41)); // 'A'
        assert!(encoding.is_valid_cp1258(0xE1)); // 'á'
        assert!(encoding.is_valid_cp1258(0x5E)); // 'đ'
        
        // Test invalid CP1258 bytes
        assert!(!encoding.is_valid_cp1258(0x80));
        assert!(!encoding.is_valid_cp1258(0xFF));
    }
}
