//! Simple encoding test for basic functionality

use unikey_encoding::Tcvn3SimpleEncoding;

fn main() {
    println!("Simple Legacy Encoding Test");
    println!("============================");
    
    let encoding = Tcvn3SimpleEncoding::new();
    
    // Test basic ASCII
    let ascii = "Hello World";
    let encoded = encoding.to_tcvn3(ascii);
    let decoded = encoding.from_tcvn3(&encoded);
    println!("ASCII: '{}' -> {:?} -> '{}'", ascii, encoded, decoded);
    assert_eq!(ascii, decoded);
    
    // Test individual Vietnamese characters
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
    
    println!("\nIndividual Vietnamese characters:");
    for ch in test_chars {
        let encoded = encoding.to_tcvn3(ch);
        let decoded = encoding.from_tcvn3(&encoded);
        println!("  '{}' -> {:?} -> '{}'", ch, encoded, decoded);
        assert_eq!(ch, decoded);
    }
    
    println!("\n=== Test completed successfully! ===");
    println!();
    println!("Legacy encoding support implemented:");
    println!("✓ TCVN3 (simplified version)");
    println!("✓ ASCII compatibility");
    println!("✓ Vietnamese character mapping");
    println!("✓ Round-trip conversion");
}
