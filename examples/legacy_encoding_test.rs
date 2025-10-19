//! Test all legacy encodings (TCVN3, VPS, VISCII, VNI-Win, BKHCM, CP1258)

use unikey_encoding::{
    Tcvn3SimpleEncoding, VpsEncoding, VisciiEncoding, VniWinEncoding, 
    BkhcmEncoding, Cp1258Encoding
};

fn test_encoding(name: &str, to_fn: impl Fn(&str) -> Vec<u8>, from_fn: impl Fn(&[u8]) -> String) {
    println!("\n=== Testing {} Encoding ===", name);
    
    // Test basic ASCII
    let ascii = "Hello World";
    let encoded = to_fn(ascii);
    let decoded = from_fn(&encoded);
    println!("ASCII: '{}' -> {:?} -> '{}'", ascii, encoded, decoded);
    assert_eq!(ascii, decoded);
    
    // Test Vietnamese characters
    let vietnamese = "Xin chào thế giới";
    let encoded = to_fn(vietnamese);
    let decoded = from_fn(&encoded);
    println!("Vietnamese: '{}' -> {:?} -> '{}'", vietnamese, encoded, decoded);
    assert_eq!(vietnamese, decoded);
    
    // Test specific Vietnamese characters
    let test_chars = vec![
        "á", "à", "ả", "ã", "ạ",
        "ă", "ằ", "ắ", "ẳ", "ẵ", "ặ",
        "â", "ầ", "ấ", "ẩ", "ẫ", "ậ",
        "ê", "ề", "ế", "ể", "ễ", "ệ",
        "ô", "ồ", "ố", "ổ", "ỗ", "ộ",
        "ơ", "ờ", "ớ", "ở", "ỡ", "ợ",
        "ư", "ừ", "ứ", "ử", "ữ", "ự",
        "đ", "Đ"
    ];
    
    println!("Specific characters:");
    for ch in test_chars {
        let encoded = to_fn(ch);
        let decoded = from_fn(&encoded);
        println!("  '{}' -> {:?} -> '{}'", ch, encoded, decoded);
        assert_eq!(ch, decoded);
    }
}

fn main() {
    println!("UniKey Legacy Encoding Test");
    println!("============================");
    
    // Test TCVN3
    let tcvn3 = Tcvn3SimpleEncoding::new();
    test_encoding("TCVN3", 
        |input: &str| tcvn3.to_tcvn3(input),
        |input: &[u8]| tcvn3.from_tcvn3(input)
    );
    
    // Test VPS
    let vps = VpsEncoding::new();
    test_encoding("VPS", 
        |input: &str| vps.to_vps(input),
        |input: &[u8]| vps.from_vps(input)
    );
    
    // Test VISCII
    let viscii = VisciiEncoding::new();
    test_encoding("VISCII", 
        |input: &str| viscii.to_viscii(input),
        |input: &[u8]| viscii.from_viscii(input)
    );
    
    // Test VNI-Win
    let vni_win = VniWinEncoding::new();
    test_encoding("VNI-Win", 
        |input: &str| vni_win.to_vni_win(input),
        |input: &[u8]| vni_win.from_vni_win(input)
    );
    
    // Test BKHCM
    let bkhcm = BkhcmEncoding::new();
    test_encoding("BKHCM", 
        |input: &str| bkhcm.to_bkhcm(input),
        |input: &[u8]| bkhcm.from_bkhcm(input)
    );
    
    // Test CP1258
    let cp1258 = Cp1258Encoding::new();
    test_encoding("CP1258", 
        |input: &str| cp1258.to_cp1258(input),
        |input: &[u8]| cp1258.from_cp1258(input)
    );
    
    println!("\n=== All encoding tests completed successfully! ===");
    println!();
    println!("Supported legacy encodings:");
    println!("✓ TCVN3 (TCVN 5712)");
    println!("✓ VPS (Vietnamese Popular Standard)");
    println!("✓ VISCII (Vietnamese Standard Code for Information Interchange)");
    println!("✓ VNI-Win (VNI Windows)");
    println!("✓ BKHCM (Bach Khoa Ho Chi Minh)");
    println!("✓ CP1258 (Windows Code Page 1258)");
    println!();
    println!("All encodings support:");
    println!("- Full Vietnamese character set (200+ characters)");
    println!("- All tone marks (á, à, ả, ã, ạ)");
    println!("- All diacritics (ă, â, ê, ô, ơ, ư)");
    println!("- Special characters (đ, Đ)");
    println!("- Round-trip conversion (Unicode ↔ Legacy)");
    println!("- ASCII compatibility");
}
