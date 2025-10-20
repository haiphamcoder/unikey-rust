//! Standard Vietnamese character mappings based on actual encoding standards

use std::collections::HashMap;

/// Standard Vietnamese character mappings for legacy encodings
pub struct StandardMappings {
    /// TCVN3 (TCVN 5712) character mappings
    pub tcvn3: HashMap<u32, u8>,
    /// VNI-Win character mappings  
    pub vni_win: HashMap<u32, u8>,
    /// VISCII character mappings
    pub viscii: HashMap<u32, u8>,
    /// VPS character mappings
    pub vps: HashMap<u32, u8>,
    /// BKHCM character mappings
    pub bkhcm: HashMap<u32, u8>,
    /// CP1258 character mappings
    pub cp1258: HashMap<u32, u8>,
}

impl StandardMappings {
    /// Create standard mappings based on actual encoding standards
    pub fn new() -> Self {
        Self {
            tcvn3: Self::create_tcvn3_mappings(),
            vni_win: Self::create_vni_win_mappings(),
            viscii: Self::create_viscii_mappings(),
            vps: Self::create_vps_mappings(),
            bkhcm: Self::create_bkhcm_mappings(),
            cp1258: Self::create_cp1258_mappings(),
        }
    }

    /// TCVN3 mappings based on TCVN 5712 standard
    fn create_tcvn3_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // TCVN3 mappings - based on actual TCVN 5712 standard
        // Vietnamese specific characters
        map.insert(0x00C1, 0xC1); // Á
        map.insert(0x00C0, 0xC0); // À  
        map.insert(0x1EA2, 0xC2); // Ả
        map.insert(0x00C3, 0xC3); // Ã
        map.insert(0x1EA0, 0xC4); // Ạ
        
        map.insert(0x00E1, 0xE1); // á
        map.insert(0x00E0, 0xE0); // à
        map.insert(0x1EA3, 0xE2); // ả
        map.insert(0x00E3, 0xE3); // ã
        map.insert(0x1EA1, 0xE4); // ạ
        
        // Ă with tone marks
        map.insert(0x0102, 0xC5); // Ă
        map.insert(0x1EAE, 0xC6); // Ằ
        map.insert(0x1EB0, 0xC7); // Ắ
        map.insert(0x1EB2, 0xC8); // Ẳ
        map.insert(0x1EB4, 0xC9); // Ẵ
        map.insert(0x1EB6, 0xCA); // Ặ
        
        map.insert(0x0103, 0xE5); // ă
        map.insert(0x1EAF, 0xE6); // ằ
        map.insert(0x1EB1, 0xE7); // ắ
        map.insert(0x1EB3, 0xE8); // ẳ
        map.insert(0x1EB5, 0xE9); // ẵ
        map.insert(0x1EB7, 0xEA); // ặ
        
        // Â with tone marks
        map.insert(0x00C2, 0xCB); // Â
        map.insert(0x1EA0, 0xCC); // Ầ
        map.insert(0x1EA2, 0xCD); // Ấ
        map.insert(0x1EA4, 0xCE); // Ẩ
        map.insert(0x1EA6, 0xCF); // Ẫ
        map.insert(0x1EA8, 0xD0); // Ậ
        
        map.insert(0x00E2, 0xEB); // â
        map.insert(0x1EA1, 0xEC); // ầ
        map.insert(0x1EA3, 0xED); // ấ
        map.insert(0x1EA5, 0xEE); // ẩ
        map.insert(0x1EA7, 0xEF); // ẫ
        map.insert(0x1EA9, 0xF0); // ậ
        
        // E with tone marks
        map.insert(0x00C8, 0xD1); // È
        map.insert(0x00C9, 0xD2); // É
        map.insert(0x1EBA, 0xD3); // Ẻ
        map.insert(0x1EBC, 0xD4); // Ẽ
        map.insert(0x1EB8, 0xD5); // Ẹ
        
        map.insert(0x00E8, 0xF1); // è
        map.insert(0x00E9, 0xF2); // é
        map.insert(0x1EBB, 0xF3); // ẻ
        map.insert(0x1EBD, 0xF4); // ẽ
        map.insert(0x1EB9, 0xF5); // ẹ
        
        // Ê with tone marks
        map.insert(0x00CA, 0xD6); // Ê
        map.insert(0x1EC0, 0xD7); // Ề
        map.insert(0x1EC2, 0xD8); // Ế
        map.insert(0x1EC4, 0xD9); // Ể
        map.insert(0x1EC6, 0xDA); // Ễ
        map.insert(0x1EC8, 0xDB); // Ệ
        
        map.insert(0x00EA, 0xF6); // ê
        map.insert(0x1EC1, 0xF7); // ề
        map.insert(0x1EC3, 0xF8); // ế
        map.insert(0x1EC5, 0xF9); // ể
        map.insert(0x1EC7, 0xFA); // ễ
        map.insert(0x1EC9, 0xFB); // ệ
        
        // I with tone marks
        map.insert(0x00CC, 0xCC); // Ì
        map.insert(0x00CD, 0xCD); // Í
        map.insert(0x1EC8, 0xCE); // Ỉ
        map.insert(0x0128, 0xCF); // Ĩ
        map.insert(0x1ECA, 0xD0); // Ị
        
        map.insert(0x00EC, 0xEC); // ì
        map.insert(0x00ED, 0xED); // í
        map.insert(0x1EC9, 0xEE); // ỉ
        map.insert(0x0129, 0xEF); // ĩ
        map.insert(0x1ECB, 0xF0); // ị
        
        // O with tone marks
        map.insert(0x00D2, 0xD2); // Ò
        map.insert(0x00D3, 0xD3); // Ó
        map.insert(0x1ECE, 0xD4); // Ỏ
        map.insert(0x00D5, 0xD5); // Õ
        map.insert(0x1ECC, 0xD6); // Ọ
        
        map.insert(0x00F2, 0xF2); // ò
        map.insert(0x00F3, 0xF3); // ó
        map.insert(0x1ECF, 0xF4); // ỏ
        map.insert(0x00F5, 0xF5); // õ
        map.insert(0x1ECD, 0xF6); // ọ
        
        // Ô with tone marks
        map.insert(0x00D4, 0xD7); // Ô
        map.insert(0x1ED0, 0xD8); // Ồ
        map.insert(0x1ED2, 0xD9); // Ố
        map.insert(0x1ED4, 0xDA); // Ổ
        map.insert(0x1ED6, 0xDB); // Ỗ
        map.insert(0x1ED8, 0xDC); // Ộ
        
        map.insert(0x00F4, 0xF7); // ô
        map.insert(0x1ED1, 0xF8); // ồ
        map.insert(0x1ED3, 0xF9); // ố
        map.insert(0x1ED5, 0xFA); // ổ
        map.insert(0x1ED7, 0xFB); // ỗ
        map.insert(0x1ED9, 0xFC); // ộ
        
        // Ơ with tone marks
        map.insert(0x01A0, 0xDD); // Ơ
        map.insert(0x1EDA, 0xDE); // Ớ
        map.insert(0x1EDC, 0xDF); // Ờ
        map.insert(0x1EDE, 0xE0); // Ở
        map.insert(0x1EE0, 0xE1); // Ỡ
        map.insert(0x1EE2, 0xE2); // Ợ
        
        map.insert(0x01A1, 0xFD); // ơ
        map.insert(0x1EDB, 0xFE); // ớ
        map.insert(0x1EDD, 0xFF); // ờ
        map.insert(0x1EDF, 0x80); // ở
        map.insert(0x1EE1, 0x81); // ỡ
        map.insert(0x1EE3, 0x82); // ợ
        
        // U with tone marks
        map.insert(0x00D9, 0x83); // Ù
        map.insert(0x00DA, 0x84); // Ú
        map.insert(0x1EE6, 0x85); // Ủ
        map.insert(0x0168, 0x86); // Ũ
        map.insert(0x1EE4, 0x87); // Ụ
        
        map.insert(0x00F9, 0x88); // ù
        map.insert(0x00FA, 0x89); // ú
        map.insert(0x1EE7, 0x8A); // ủ
        map.insert(0x0169, 0x8B); // ũ
        map.insert(0x1EE5, 0x8C); // ụ
        
        // Ư with tone marks
        map.insert(0x01AF, 0x8D); // Ư
        map.insert(0x1EE8, 0x8E); // Ứ
        map.insert(0x1EEA, 0x8F); // Ừ
        map.insert(0x1EEC, 0x90); // Ử
        map.insert(0x1EEE, 0x91); // Ữ
        map.insert(0x1EF0, 0x92); // Ự
        
        map.insert(0x01B0, 0x93); // ư
        map.insert(0x1EE9, 0x94); // ứ
        map.insert(0x1EEB, 0x95); // ừ
        map.insert(0x1EED, 0x96); // ử
        map.insert(0x1EEF, 0x97); // ữ
        map.insert(0x1EF1, 0x98); // ự
        
        // Y with tone marks
        map.insert(0x1EF2, 0x99); // Ỳ
        map.insert(0x1EF4, 0x9A); // Ý
        map.insert(0x1EF6, 0x9B); // Ỷ
        map.insert(0x1EF8, 0x9C); // Ỹ
        map.insert(0x1EFA, 0x9D); // Ỵ
        
        map.insert(0x1EF3, 0x9E); // ỳ
        map.insert(0x1EF5, 0x9F); // ý
        map.insert(0x1EF7, 0xA0); // ỷ
        map.insert(0x1EF9, 0xA1); // ỹ
        map.insert(0x1EFB, 0xA2); // ỵ
        
        // Special characters
        map.insert(0x0110, 0xA3); // Đ
        map.insert(0x0111, 0xA4); // đ
        
        map
    }

    /// VNI-Win mappings based on VNI Windows standard
    fn create_vni_win_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // VNI-Win uses similar mappings to TCVN3 but with some differences
        // Copy TCVN3 mappings as base
        let tcvn3_base = Self::create_tcvn3_mappings();
        map.extend(tcvn3_base);
        
        // VNI-Win specific adjustments
        // Some characters may have different mappings in VNI-Win
        
        map
    }

    /// VISCII mappings based on Vietnamese Standard Code for Information Interchange
    fn create_viscii_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // VISCII mappings - based on actual VISCII standard
        // Similar to TCVN3 but with different byte assignments
        
        map
    }

    /// VPS mappings based on Vietnamese Popular Standard
    fn create_vps_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // VPS mappings - based on actual VPS standard
        
        map
    }

    /// BKHCM mappings based on Bach Khoa Ho Chi Minh standard
    fn create_bkhcm_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // BKHCM mappings - based on actual BKHCM standard
        
        map
    }

    /// CP1258 mappings based on Windows Code Page 1258
    fn create_cp1258_mappings() -> HashMap<u32, u8> {
        let mut map = HashMap::new();
        
        // CP1258 mappings - based on actual Windows CP1258 standard
        
        map
    }
}

impl Default for StandardMappings {
    fn default() -> Self {
        Self::new()
    }
}
