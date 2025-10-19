//! Vietnamese character mappings and conversion tables

use crate::{VnLexiName, VowelSeq, KeyEventType, CharType};

/// Vietnamese character to Unicode mapping
pub struct VnCharMapping {
    /// Unicode code point
    pub unicode: u32,
    /// Character name
    pub name: &'static str,
    /// Telex input sequence
    pub telex: &'static str,
    /// VNI input sequence  
    pub vni: &'static str,
    /// VIQR input sequence
    pub viqr: &'static str,
}

/// Vietnamese character mapping table
pub const VN_CHAR_MAP: &[VnCharMapping] = &[
    // Base characters
    VnCharMapping { unicode: 0x0041, name: "A", telex: "A", vni: "A", viqr: "A" },
    VnCharMapping { unicode: 0x0061, name: "a", telex: "a", vni: "a", viqr: "a" },
    VnCharMapping { unicode: 0x0042, name: "B", telex: "B", vni: "B", viqr: "B" },
    VnCharMapping { unicode: 0x0062, name: "b", telex: "b", vni: "b", viqr: "b" },
    VnCharMapping { unicode: 0x0043, name: "C", telex: "C", vni: "C", viqr: "C" },
    VnCharMapping { unicode: 0x0063, name: "c", telex: "c", vni: "c", viqr: "c" },
    VnCharMapping { unicode: 0x0044, name: "D", telex: "D", vni: "D", viqr: "D" },
    VnCharMapping { unicode: 0x0064, name: "d", telex: "d", vni: "d", viqr: "d" },
    VnCharMapping { unicode: 0x0045, name: "E", telex: "E", vni: "E", viqr: "E" },
    VnCharMapping { unicode: 0x0065, name: "e", telex: "e", vni: "e", viqr: "e" },
    VnCharMapping { unicode: 0x0046, name: "F", telex: "F", vni: "F", viqr: "F" },
    VnCharMapping { unicode: 0x0066, name: "f", telex: "f", vni: "f", viqr: "f" },
    VnCharMapping { unicode: 0x0047, name: "G", telex: "G", vni: "G", viqr: "G" },
    VnCharMapping { unicode: 0x0067, name: "g", telex: "g", vni: "g", viqr: "g" },
    VnCharMapping { unicode: 0x0048, name: "H", telex: "H", vni: "H", viqr: "H" },
    VnCharMapping { unicode: 0x0068, name: "h", telex: "h", vni: "h", viqr: "h" },
    VnCharMapping { unicode: 0x0049, name: "I", telex: "I", vni: "I", viqr: "I" },
    VnCharMapping { unicode: 0x0069, name: "i", telex: "i", vni: "i", viqr: "i" },
    VnCharMapping { unicode: 0x004A, name: "J", telex: "J", vni: "J", viqr: "J" },
    VnCharMapping { unicode: 0x006A, name: "j", telex: "j", vni: "j", viqr: "j" },
    VnCharMapping { unicode: 0x004B, name: "K", telex: "K", vni: "K", viqr: "K" },
    VnCharMapping { unicode: 0x006B, name: "k", telex: "k", vni: "k", viqr: "k" },
    VnCharMapping { unicode: 0x004C, name: "L", telex: "L", vni: "L", viqr: "L" },
    VnCharMapping { unicode: 0x006C, name: "l", telex: "l", vni: "l", viqr: "l" },
    VnCharMapping { unicode: 0x004D, name: "M", telex: "M", vni: "M", viqr: "M" },
    VnCharMapping { unicode: 0x006D, name: "m", telex: "m", vni: "m", viqr: "m" },
    VnCharMapping { unicode: 0x004E, name: "N", telex: "N", vni: "N", viqr: "N" },
    VnCharMapping { unicode: 0x006E, name: "n", telex: "n", vni: "n", viqr: "n" },
    VnCharMapping { unicode: 0x004F, name: "O", telex: "O", vni: "O", viqr: "O" },
    VnCharMapping { unicode: 0x006F, name: "o", telex: "o", vni: "o", viqr: "o" },
    VnCharMapping { unicode: 0x0050, name: "P", telex: "P", vni: "P", viqr: "P" },
    VnCharMapping { unicode: 0x0070, name: "p", telex: "p", vni: "p", viqr: "p" },
    VnCharMapping { unicode: 0x0051, name: "Q", telex: "Q", vni: "Q", viqr: "Q" },
    VnCharMapping { unicode: 0x0071, name: "q", telex: "q", vni: "q", viqr: "q" },
    VnCharMapping { unicode: 0x0052, name: "R", telex: "R", vni: "R", viqr: "R" },
    VnCharMapping { unicode: 0x0072, name: "r", telex: "r", vni: "r", viqr: "r" },
    VnCharMapping { unicode: 0x0053, name: "S", telex: "S", vni: "S", viqr: "S" },
    VnCharMapping { unicode: 0x0073, name: "s", telex: "s", vni: "s", viqr: "s" },
    VnCharMapping { unicode: 0x0054, name: "T", telex: "T", vni: "T", viqr: "T" },
    VnCharMapping { unicode: 0x0074, name: "t", telex: "t", vni: "t", viqr: "t" },
    VnCharMapping { unicode: 0x0055, name: "U", telex: "U", vni: "U", viqr: "U" },
    VnCharMapping { unicode: 0x0075, name: "u", telex: "u", vni: "u", viqr: "u" },
    VnCharMapping { unicode: 0x0056, name: "V", telex: "V", vni: "V", viqr: "V" },
    VnCharMapping { unicode: 0x0076, name: "v", telex: "v", vni: "v", viqr: "v" },
    VnCharMapping { unicode: 0x0057, name: "W", telex: "W", vni: "W", viqr: "W" },
    VnCharMapping { unicode: 0x0077, name: "w", telex: "w", vni: "w", viqr: "w" },
    VnCharMapping { unicode: 0x0058, name: "X", telex: "X", vni: "X", viqr: "X" },
    VnCharMapping { unicode: 0x0078, name: "x", telex: "x", vni: "x", viqr: "x" },
    VnCharMapping { unicode: 0x0059, name: "Y", telex: "Y", vni: "Y", viqr: "Y" },
    VnCharMapping { unicode: 0x0079, name: "y", telex: "y", vni: "y", viqr: "y" },
    VnCharMapping { unicode: 0x005A, name: "Z", telex: "Z", vni: "Z", viqr: "Z" },
    VnCharMapping { unicode: 0x007A, name: "z", telex: "z", vni: "z", viqr: "z" },
    
    // Vietnamese specific characters
    VnCharMapping { unicode: 0x0110, name: "Đ", telex: "DD", vni: "D9", viqr: "DD" },
    VnCharMapping { unicode: 0x0111, name: "đ", telex: "dd", vni: "d9", viqr: "dd" },
    
    // Vowels with tone marks
    VnCharMapping { unicode: 0x00C0, name: "À", telex: "A2", vni: "A2", viqr: "A`" },
    VnCharMapping { unicode: 0x00E0, name: "à", telex: "a2", vni: "a2", viqr: "a`" },
    VnCharMapping { unicode: 0x00C1, name: "Á", telex: "A1", vni: "A1", viqr: "A'" },
    VnCharMapping { unicode: 0x00E1, name: "á", telex: "a1", vni: "a1", viqr: "a'" },
    VnCharMapping { unicode: 0x1EA2, name: "Ả", telex: "A3", vni: "A3", viqr: "A?" },
    VnCharMapping { unicode: 0x1EA3, name: "ả", telex: "a3", vni: "a3", viqr: "a?" },
    VnCharMapping { unicode: 0x00C3, name: "Ã", telex: "A4", vni: "A4", viqr: "A~" },
    VnCharMapping { unicode: 0x00E3, name: "ã", telex: "a4", vni: "a4", viqr: "a~" },
    VnCharMapping { unicode: 0x1EA0, name: "Ạ", telex: "A5", vni: "A5", viqr: "A." },
    VnCharMapping { unicode: 0x1EA1, name: "ạ", telex: "a5", vni: "a5", viqr: "a." },
    
    // Ă with tone marks
    VnCharMapping { unicode: 0x1EA0, name: "Ằ", telex: "Ar2", vni: "A82", viqr: "A(`" },
    VnCharMapping { unicode: 0x1EA1, name: "ằ", telex: "ar2", vni: "a82", viqr: "a(`" },
    VnCharMapping { unicode: 0x1EA2, name: "Ắ", telex: "Ar1", vni: "A81", viqr: "A('" },
    VnCharMapping { unicode: 0x1EA3, name: "ắ", telex: "ar1", vni: "a81", viqr: "a('" },
    VnCharMapping { unicode: 0x1EA4, name: "Ẳ", telex: "Ar3", vni: "A83", viqr: "A(?" },
    VnCharMapping { unicode: 0x1EA5, name: "ẳ", telex: "ar3", vni: "a83", viqr: "a(?" },
    VnCharMapping { unicode: 0x1EA6, name: "Ẵ", telex: "Ar4", vni: "A84", viqr: "A(~" },
    VnCharMapping { unicode: 0x1EA7, name: "ẵ", telex: "ar4", vni: "a84", viqr: "a(~" },
    VnCharMapping { unicode: 0x1EA8, name: "Ặ", telex: "Ar5", vni: "A85", viqr: "A(." },
    VnCharMapping { unicode: 0x1EA9, name: "ặ", telex: "ar5", vni: "a85", viqr: "a(." },
    
    // Base Ă without tone marks
    VnCharMapping { unicode: 0x0102, name: "Ă", telex: "Ar", vni: "A8", viqr: "A(" },
    VnCharMapping { unicode: 0x0103, name: "ă", telex: "ar", vni: "a8", viqr: "a(" },
    
    // Alternative Telex sequences for Ă
    VnCharMapping { unicode: 0x0102, name: "Ă", telex: "aw", vni: "A8", viqr: "A(" },
    VnCharMapping { unicode: 0x0103, name: "ă", telex: "aw", vni: "a8", viqr: "a(" },
    
    // Â with tone marks
    VnCharMapping { unicode: 0x1EA0, name: "Ầ", telex: "Ab2", vni: "A62", viqr: "A^`" },
    VnCharMapping { unicode: 0x1EA1, name: "ầ", telex: "ab2", vni: "a62", viqr: "a^`" },
    VnCharMapping { unicode: 0x1EA2, name: "Ấ", telex: "Ab1", vni: "A61", viqr: "A^'" },
    VnCharMapping { unicode: 0x1EA3, name: "ấ", telex: "ab1", vni: "a61", viqr: "a^'" },
    VnCharMapping { unicode: 0x1EA4, name: "Ẩ", telex: "Ab3", vni: "A63", viqr: "A^?" },
    VnCharMapping { unicode: 0x1EA5, name: "ẩ", telex: "ab3", vni: "a63", viqr: "a^?" },
    VnCharMapping { unicode: 0x1EA6, name: "Ẫ", telex: "Ab4", vni: "A64", viqr: "A^~" },
    VnCharMapping { unicode: 0x1EA7, name: "ẫ", telex: "ab4", vni: "a64", viqr: "a^~" },
    VnCharMapping { unicode: 0x1EA8, name: "Ậ", telex: "Ab5", vni: "A65", viqr: "A^." },
    VnCharMapping { unicode: 0x1EA9, name: "ậ", telex: "ab5", vni: "a65", viqr: "a^." },
    
    // Base Â without tone marks
    VnCharMapping { unicode: 0x00C2, name: "Â", telex: "Ab", vni: "A6", viqr: "A^" },
    VnCharMapping { unicode: 0x00E2, name: "â", telex: "ab", vni: "a6", viqr: "a^" },
    
    // Alternative Telex sequences for Â
    VnCharMapping { unicode: 0x00C2, name: "Â", telex: "aa", vni: "A6", viqr: "A^" },
    VnCharMapping { unicode: 0x00E2, name: "â", telex: "aa", vni: "a6", viqr: "a^" },
    
    // E with tone marks
    VnCharMapping { unicode: 0x00C8, name: "È", telex: "E2", vni: "E2", viqr: "E`" },
    VnCharMapping { unicode: 0x00E8, name: "è", telex: "e2", vni: "e2", viqr: "e`" },
    VnCharMapping { unicode: 0x00C9, name: "É", telex: "E1", vni: "E1", viqr: "E'" },
    VnCharMapping { unicode: 0x00E9, name: "é", telex: "e1", vni: "e1", viqr: "e'" },
    VnCharMapping { unicode: 0x1EBA, name: "Ẻ", telex: "E3", vni: "E3", viqr: "E?" },
    VnCharMapping { unicode: 0x1EBB, name: "ẻ", telex: "e3", vni: "e3", viqr: "e?" },
    VnCharMapping { unicode: 0x1EBC, name: "Ẽ", telex: "E4", vni: "E4", viqr: "E~" },
    VnCharMapping { unicode: 0x1EBD, name: "ẽ", telex: "e4", vni: "e4", viqr: "e~" },
    VnCharMapping { unicode: 0x1EB8, name: "Ẹ", telex: "E5", vni: "E5", viqr: "E." },
    VnCharMapping { unicode: 0x1EB9, name: "ẹ", telex: "e5", vni: "e5", viqr: "e." },
    
    // Ê with tone marks
    VnCharMapping { unicode: 0x1EBC, name: "Ề", telex: "Er2", vni: "E82", viqr: "E^`" },
    VnCharMapping { unicode: 0x1EBD, name: "ề", telex: "er2", vni: "e82", viqr: "e^`" },
    VnCharMapping { unicode: 0x1EBE, name: "Ế", telex: "Er1", vni: "E81", viqr: "E^'" },
    VnCharMapping { unicode: 0x1EBF, name: "ế", telex: "er1", vni: "e81", viqr: "e^'" },
    VnCharMapping { unicode: 0x1EC0, name: "Ể", telex: "Er3", vni: "E83", viqr: "E^?" },
    VnCharMapping { unicode: 0x1EC1, name: "ể", telex: "er3", vni: "e83", viqr: "e^?" },
    VnCharMapping { unicode: 0x1EC2, name: "Ễ", telex: "Er4", vni: "E84", viqr: "E^~" },
    VnCharMapping { unicode: 0x1EC3, name: "ễ", telex: "er4", vni: "e84", viqr: "e^~" },
    VnCharMapping { unicode: 0x1EC4, name: "Ệ", telex: "Er5", vni: "E85", viqr: "E^." },
    VnCharMapping { unicode: 0x1EC5, name: "ệ", telex: "er5", vni: "e85", viqr: "e^." },
    
    // Base Ê without tone marks
    VnCharMapping { unicode: 0x00CA, name: "Ê", telex: "Er", vni: "E8", viqr: "E^" },
    VnCharMapping { unicode: 0x00EA, name: "ê", telex: "er", vni: "e8", viqr: "e^" },
    
    // Alternative Telex sequences for Ê
    VnCharMapping { unicode: 0x00CA, name: "Ê", telex: "ee", vni: "E8", viqr: "E^" },
    VnCharMapping { unicode: 0x00EA, name: "ê", telex: "ee", vni: "e8", viqr: "e^" },
    
    // I with tone marks
    VnCharMapping { unicode: 0x00CC, name: "Ì", telex: "I2", vni: "I2", viqr: "I`" },
    VnCharMapping { unicode: 0x00EC, name: "ì", telex: "i2", vni: "i2", viqr: "i`" },
    VnCharMapping { unicode: 0x00CD, name: "Í", telex: "I1", vni: "I1", viqr: "I'" },
    VnCharMapping { unicode: 0x00ED, name: "í", telex: "i1", vni: "i1", viqr: "i'" },
    VnCharMapping { unicode: 0x1EC8, name: "Ỉ", telex: "I3", vni: "I3", viqr: "I?" },
    VnCharMapping { unicode: 0x1EC9, name: "ỉ", telex: "i3", vni: "i3", viqr: "i?" },
    VnCharMapping { unicode: 0x0128, name: "Ĩ", telex: "I4", vni: "I4", viqr: "I~" },
    VnCharMapping { unicode: 0x0129, name: "ĩ", telex: "i4", vni: "i4", viqr: "i~" },
    VnCharMapping { unicode: 0x1ECA, name: "Ị", telex: "I5", vni: "I5", viqr: "I." },
    VnCharMapping { unicode: 0x1ECB, name: "ị", telex: "i5", vni: "i5", viqr: "i." },
    
    // O with tone marks
    VnCharMapping { unicode: 0x00D2, name: "Ò", telex: "O2", vni: "O2", viqr: "O`" },
    VnCharMapping { unicode: 0x00F2, name: "ò", telex: "o2", vni: "o2", viqr: "o`" },
    VnCharMapping { unicode: 0x00D3, name: "Ó", telex: "O1", vni: "O1", viqr: "O'" },
    VnCharMapping { unicode: 0x00F3, name: "ó", telex: "o1", vni: "o1", viqr: "o'" },
    VnCharMapping { unicode: 0x1ECE, name: "Ỏ", telex: "O3", vni: "O3", viqr: "O?" },
    VnCharMapping { unicode: 0x1ECF, name: "ỏ", telex: "o3", vni: "o3", viqr: "o?" },
    VnCharMapping { unicode: 0x00D5, name: "Õ", telex: "O4", vni: "O4", viqr: "O~" },
    VnCharMapping { unicode: 0x00F5, name: "õ", telex: "o4", vni: "o4", viqr: "o~" },
    VnCharMapping { unicode: 0x1ECC, name: "Ọ", telex: "O5", vni: "O5", viqr: "O." },
    VnCharMapping { unicode: 0x1ECD, name: "ọ", telex: "o5", vni: "o5", viqr: "o." },
    
    // Ô with tone marks
    VnCharMapping { unicode: 0x1ED0, name: "Ồ", telex: "Or2", vni: "O62", viqr: "O^`" },
    VnCharMapping { unicode: 0x1ED1, name: "ồ", telex: "or2", vni: "o62", viqr: "o^`" },
    VnCharMapping { unicode: 0x1ED2, name: "Ố", telex: "Or1", vni: "O61", viqr: "O^'" },
    VnCharMapping { unicode: 0x1ED3, name: "ố", telex: "or1", vni: "o61", viqr: "o^'" },
    VnCharMapping { unicode: 0x1ED4, name: "Ổ", telex: "Or3", vni: "O63", viqr: "O^?" },
    VnCharMapping { unicode: 0x1ED5, name: "ổ", telex: "or3", vni: "o63", viqr: "o^?" },
    VnCharMapping { unicode: 0x1ED6, name: "Ỗ", telex: "Or4", vni: "O64", viqr: "O^~" },
    VnCharMapping { unicode: 0x1ED7, name: "ỗ", telex: "or4", vni: "o64", viqr: "o^~" },
    VnCharMapping { unicode: 0x1ED8, name: "Ộ", telex: "Or5", vni: "O65", viqr: "O^." },
    VnCharMapping { unicode: 0x1ED9, name: "ộ", telex: "or5", vni: "o65", viqr: "o^." },
    
    // Base Ô without tone marks
    VnCharMapping { unicode: 0x00D4, name: "Ô", telex: "Or", vni: "O6", viqr: "O^" },
    VnCharMapping { unicode: 0x00F4, name: "ô", telex: "or", vni: "o6", viqr: "o^" },
    
    // Alternative Telex sequences for Ô
    VnCharMapping { unicode: 0x00D4, name: "Ô", telex: "oo", vni: "O6", viqr: "O^" },
    VnCharMapping { unicode: 0x00F4, name: "ô", telex: "oo", vni: "o6", viqr: "o^" },
    
    // Ơ with tone marks
    VnCharMapping { unicode: 0x1EDA, name: "Ớ", telex: "Oh1", vni: "O71", viqr: "O+'", },
    VnCharMapping { unicode: 0x1EDB, name: "ớ", telex: "oh1", vni: "o71", viqr: "o+'" },
    VnCharMapping { unicode: 0x1EDC, name: "Ờ", telex: "Oh2", vni: "O72", viqr: "O+`" },
    VnCharMapping { unicode: 0x1EDD, name: "ờ", telex: "oh2", vni: "o72", viqr: "o+`" },
    VnCharMapping { unicode: 0x1EDE, name: "Ở", telex: "Oh3", vni: "O73", viqr: "O+?" },
    VnCharMapping { unicode: 0x1EDF, name: "ở", telex: "oh3", vni: "o73", viqr: "o+?" },
    VnCharMapping { unicode: 0x1EE0, name: "Ỡ", telex: "Oh4", vni: "O74", viqr: "O+~" },
    VnCharMapping { unicode: 0x1EE1, name: "ỡ", telex: "oh4", vni: "o74", viqr: "o+~" },
    VnCharMapping { unicode: 0x1EE2, name: "Ợ", telex: "Oh5", vni: "O75", viqr: "O+." },
    VnCharMapping { unicode: 0x1EE3, name: "ợ", telex: "oh5", vni: "o75", viqr: "o+." },
    
    // Base Ơ without tone marks
    VnCharMapping { unicode: 0x01A0, name: "Ơ", telex: "Oh", vni: "O7", viqr: "O+" },
    VnCharMapping { unicode: 0x01A1, name: "ơ", telex: "oh", vni: "o7", viqr: "o+" },
    
    // Alternative Telex sequences for Ơ
    VnCharMapping { unicode: 0x01A0, name: "Ơ", telex: "ow", vni: "O7", viqr: "O+" },
    VnCharMapping { unicode: 0x01A1, name: "ơ", telex: "ow", vni: "o7", viqr: "o+" },
    
    // U with tone marks
    VnCharMapping { unicode: 0x00D9, name: "Ù", telex: "U2", vni: "U2", viqr: "U`" },
    VnCharMapping { unicode: 0x00F9, name: "ù", telex: "u2", vni: "u2", viqr: "u`" },
    VnCharMapping { unicode: 0x00DA, name: "Ú", telex: "U1", vni: "U1", viqr: "U'" },
    VnCharMapping { unicode: 0x00FA, name: "ú", telex: "u1", vni: "u1", viqr: "u'" },
    VnCharMapping { unicode: 0x1EE6, name: "Ủ", telex: "U3", vni: "U3", viqr: "U?" },
    VnCharMapping { unicode: 0x1EE7, name: "ủ", telex: "u3", vni: "u3", viqr: "u?" },
    VnCharMapping { unicode: 0x0168, name: "Ũ", telex: "U4", vni: "U4", viqr: "U~" },
    VnCharMapping { unicode: 0x0169, name: "ũ", telex: "u4", vni: "u4", viqr: "u~" },
    VnCharMapping { unicode: 0x1EE4, name: "Ụ", telex: "U5", vni: "U5", viqr: "U." },
    VnCharMapping { unicode: 0x1EE5, name: "ụ", telex: "u5", vni: "u5", viqr: "u." },
    
    // Ư with tone marks
    VnCharMapping { unicode: 0x1EE8, name: "Ứ", telex: "Uh1", vni: "U71", viqr: "U+'", },
    VnCharMapping { unicode: 0x1EE9, name: "ứ", telex: "uh1", vni: "u71", viqr: "u+'" },
    VnCharMapping { unicode: 0x1EEA, name: "Ừ", telex: "Uh2", vni: "U72", viqr: "U+`" },
    VnCharMapping { unicode: 0x1EEB, name: "ừ", telex: "uh2", vni: "u72", viqr: "u+`" },
    VnCharMapping { unicode: 0x1EEC, name: "Ử", telex: "Uh3", vni: "U73", viqr: "U+?" },
    VnCharMapping { unicode: 0x1EED, name: "ử", telex: "uh3", vni: "u73", viqr: "u+?" },
    VnCharMapping { unicode: 0x1EEE, name: "Ữ", telex: "Uh4", vni: "U74", viqr: "U+~" },
    VnCharMapping { unicode: 0x1EEF, name: "ữ", telex: "uh4", vni: "u74", viqr: "u+~" },
    VnCharMapping { unicode: 0x1EF0, name: "Ự", telex: "Uh5", vni: "U75", viqr: "U+." },
    VnCharMapping { unicode: 0x1EF1, name: "ự", telex: "uh5", vni: "u75", viqr: "u+." },
    
    // Base Ư without tone marks
    VnCharMapping { unicode: 0x01AF, name: "Ư", telex: "Uh", vni: "U7", viqr: "U+" },
    VnCharMapping { unicode: 0x01B0, name: "ư", telex: "uh", vni: "u7", viqr: "u+" },
    
    // Alternative Telex sequences for Ư
    VnCharMapping { unicode: 0x01AF, name: "Ư", telex: "uw", vni: "U7", viqr: "U+" },
    VnCharMapping { unicode: 0x01B0, name: "ư", telex: "uw", vni: "u7", viqr: "u+" },
    
    // Y with tone marks
    VnCharMapping { unicode: 0x1EF2, name: "Ỳ", telex: "Y2", vni: "Y2", viqr: "Y`" },
    VnCharMapping { unicode: 0x1EF3, name: "ỳ", telex: "y2", vni: "y2", viqr: "y`" },
    VnCharMapping { unicode: 0x1EF4, name: "Ỵ", telex: "Y1", vni: "Y1", viqr: "Y'" },
    VnCharMapping { unicode: 0x1EF5, name: "ý", telex: "y1", vni: "y1", viqr: "y'" },
    VnCharMapping { unicode: 0x1EF6, name: "Ỷ", telex: "Y3", vni: "Y3", viqr: "Y?" },
    VnCharMapping { unicode: 0x1EF7, name: "ỷ", telex: "y3", vni: "y3", viqr: "y?" },
    VnCharMapping { unicode: 0x1EF8, name: "Ỹ", telex: "Y4", vni: "Y4", viqr: "Y~" },
    VnCharMapping { unicode: 0x1EF9, name: "ỹ", telex: "y4", vni: "y4", viqr: "y~" },
    VnCharMapping { unicode: 0x1EFA, name: "Ỳ", telex: "Y5", vni: "Y5", viqr: "Y." },
    VnCharMapping { unicode: 0x1EFB, name: "ỵ", telex: "y5", vni: "y5", viqr: "y." },
];

/// Character mapping utilities
pub struct CharMapper;

impl CharMapper {
    /// Find character mapping by Unicode code point
    pub fn find_by_unicode(unicode: u32) -> Option<&'static VnCharMapping> {
        VN_CHAR_MAP.iter().find(|m| m.unicode == unicode)
    }
    
    /// Find character mapping by Telex sequence
    pub fn find_by_telex(telex: &str) -> Option<&'static VnCharMapping> {
        VN_CHAR_MAP.iter().find(|m| m.telex == telex)
    }
    
    /// Find character mapping by VNI sequence
    pub fn find_by_vni(vni: &str) -> Option<&'static VnCharMapping> {
        VN_CHAR_MAP.iter().find(|m| m.vni == vni)
    }
    
    /// Find character mapping by VIQR sequence
    pub fn find_by_viqr(viqr: &str) -> Option<&'static VnCharMapping> {
        VN_CHAR_MAP.iter().find(|m| m.viqr == viqr)
    }
    
    /// Convert Unicode to Telex sequence
    pub fn unicode_to_telex(unicode: u32) -> Option<&'static str> {
        Self::find_by_unicode(unicode).map(|m| m.telex)
    }
    
    /// Convert Unicode to VNI sequence
    pub fn unicode_to_vni(unicode: u32) -> Option<&'static str> {
        Self::find_by_unicode(unicode).map(|m| m.vni)
    }
    
    /// Convert Unicode to VIQR sequence
    pub fn unicode_to_viqr(unicode: u32) -> Option<&'static str> {
        Self::find_by_unicode(unicode).map(|m| m.viqr)
    }
    
    /// Convert Telex sequence to Unicode
    pub fn telex_to_unicode(telex: &str) -> Option<u32> {
        Self::find_by_telex(telex).map(|m| m.unicode)
    }
    
    /// Convert VNI sequence to Unicode
    pub fn vni_to_unicode(vni: &str) -> Option<u32> {
        Self::find_by_vni(vni).map(|m| m.unicode)
    }
    
    /// Convert VIQR sequence to Unicode
    pub fn viqr_to_unicode(viqr: &str) -> Option<u32> {
        Self::find_by_viqr(viqr).map(|m| m.unicode)
    }
}

/// Word break characters
pub const WORD_BREAK_CHARS: &[char] = &[
    ' ', '\t', '\n', '\r',  // Whitespace
    '.', ',', ';', ':', '!', '?',  // Punctuation
    '(', ')', '[', ']', '{', '}',  // Brackets
    '"', '\'', '`',  // Quotes
    '-', '_', '=', '+', '*', '/', '\\',  // Operators
    '|', '&', '%', '$', '#', '@',  // Special characters
    '<', '>',  // Comparison
];

/// Check if a character is a word break character
pub fn is_word_break_char(ch: char) -> bool {
    WORD_BREAK_CHARS.contains(&ch)
}

/// Check if a character is a Vietnamese character
pub fn is_vietnamese_char(ch: char) -> bool {
    let unicode = ch as u32;
    VN_CHAR_MAP.iter().any(|m| m.unicode == unicode)
}

/// Get the tone number from a Vietnamese character
pub fn get_tone_from_char(ch: char) -> Option<u8> {
    let unicode = ch as u32;
    if let Some(mapping) = CharMapper::find_by_unicode(unicode) {
        // Extract tone from the sequence
        if mapping.telex.len() > 1 {
            if let Some(last_char) = mapping.telex.chars().last() {
                if last_char.is_ascii_digit() {
                    return Some(last_char.to_digit(10).unwrap() as u8);
                }
            }
        }
    }
    None
}
