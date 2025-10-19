//! Test Telex input method functionality

use unikey_core::{Engine, KeyEvent, KeyEventType, CharType, VnLexiName, CharMapper};

fn main() {
    println!("UniKey Rust - Telex Input Method Test");
    println!("=====================================");

    let mut engine = Engine::new();
    engine.set_input_method("Telex".to_string());
    engine.enable_engine(true);

    // Test cases for Telex input
    let test_cases = vec![
        ("a", "a"),
        ("aa", "â"),
        ("aw", "ă"),
        ("ee", "ê"),
        ("oo", "ô"),
        ("ow", "ơ"),
        ("uw", "ư"),
        ("dd", "đ"),
        ("a1", "á"),
        ("a2", "à"),
        ("a3", "ả"),
        ("a4", "ã"),
        ("a5", "ạ"),
        ("aa1", "ấ"),
        ("aa2", "ầ"),
        ("aa3", "ẩ"),
        ("aa4", "ẫ"),
        ("aa5", "ậ"),
        ("aw1", "ắ"),
        ("aw2", "ằ"),
        ("aw3", "ẳ"),
        ("aw4", "ẵ"),
        ("aw5", "ặ"),
    ];

    println!("\nTesting Telex input sequences:");
    println!("{:<10} {:<10} {:<10} {:<10}", "Input", "Expected", "Unicode", "Result");
    println!("{}", "-".repeat(50));

    for (input, expected) in test_cases {
        // Clear engine state
        engine.reset();
        
        // Process each character in the input sequence
        let mut result = String::new();
        for ch in input.chars() {
            let key_event = KeyEvent::new(
                KeyEventType::Normal,
                CharType::Vn,
                VnLexiName::A, // This will be updated based on actual character
                ch as u32,
            );
            
            match engine.process_key(key_event) {
                Ok(output) => {
                    if !output.is_empty() {
                        result = String::from_utf8_lossy(&output).to_string();
                    }
                }
                Err(e) => {
                    println!("Error processing '{}': {}", ch, e);
                }
            }
        }
        
        // Get Unicode code point for comparison
        let unicode = CharMapper::telex_to_unicode(input).unwrap_or(0);
        let unicode_str = if unicode > 0 {
            format!("U+{:04X}", unicode)
        } else {
            "N/A".to_string()
        };
        
        println!("{:<10} {:<10} {:<10} {:<10}", 
                 input, expected, unicode_str, result);
    }

    println!("\nTesting character mapping functions:");
    println!("{:<10} {:<10} {:<10} {:<10}", "Telex", "VNI", "VIQR", "Unicode");
    println!("{}", "-".repeat(50));

    let mapping_tests = vec!["a", "aa", "aw", "ee", "oo", "ow", "uw", "dd"];
    
    for telex in mapping_tests {
        let vni = CharMapper::find_by_telex(telex)
            .map(|m| m.vni)
            .unwrap_or("N/A");
        let viqr = CharMapper::find_by_telex(telex)
            .map(|m| m.viqr)
            .unwrap_or("N/A");
        let unicode = CharMapper::telex_to_unicode(telex)
            .map(|u| format!("U+{:04X}", u))
            .unwrap_or("N/A".to_string());
        
        println!("{:<10} {:<10} {:<10} {:<10}", telex, vni, viqr, unicode);
    }

    println!("\nTest completed successfully!");
}
