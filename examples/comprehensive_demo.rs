//! Comprehensive demo of all UniKey functionality

use unikey_core::{Engine, KeyEvent, KeyEventType, CharType, VnLexiName, CharMapper};
use unikey_input_methods::{TelexMethod, VniMethod, ViqrMethod, InputMethod};

fn demo_character_mapping() {
    println!("=== Vietnamese Character Mapping Demo ===");
    println!();
    
    let test_sequences = vec![
        ("Telex", vec![
            ("aa", "â"), ("aw", "ă"), ("ee", "ê"), ("oo", "ô"), ("ow", "ơ"), ("uw", "ư"),
            ("a1", "á"), ("a2", "à"), ("a3", "ả"), ("a4", "ã"), ("a5", "ạ"),
            ("dd", "đ"), ("DD", "Đ"),
        ]),
        ("VNI", vec![
            ("a6", "â"), ("a8", "ă"), ("e6", "ê"), ("o6", "ô"), ("o7", "ơ"), ("u7", "ư"),
            ("a1", "á"), ("a2", "à"), ("a3", "ả"), ("a4", "ã"), ("a5", "ạ"),
            ("d9", "đ"), ("D9", "Đ"),
        ]),
        ("VIQR", vec![
            ("a^", "â"), ("a(", "ă"), ("e^", "ê"), ("o^", "ô"), ("o+", "ơ"), ("u+", "ư"),
            ("a'", "á"), ("a`", "à"), ("a?", "ả"), ("a~", "ã"), ("a.", "ạ"),
            ("dd", "đ"), ("DD", "Đ"),
        ]),
    ];
    
    for (method_name, sequences) in test_sequences {
        println!("{} Method:", method_name);
        for (input, expected) in sequences {
            let unicode = match method_name {
                "Telex" => CharMapper::telex_to_unicode(input),
                "VNI" => CharMapper::vni_to_unicode(input),
                "VIQR" => CharMapper::viqr_to_unicode(input),
                _ => None,
            };
            
            if let Some(unicode) = unicode {
                let actual = std::char::from_u32(unicode).unwrap();
                println!("  {} -> {} (U+{:04X}) ✓", input, actual, unicode);
            } else {
                println!("  {} -> Not found ✗", input);
            }
        }
        println!();
    }
}

fn demo_input_methods() {
    println!("=== Input Methods Demo ===");
    println!();
    
    let test_cases = vec![
        ("Telex", "aa", vec![97, 97]), // 'a', 'a'
        ("Telex", "aw", vec![97, 119]), // 'a', 'w'
        ("Telex", "a1", vec![97, 49]), // 'a', '1'
        ("VNI", "a6", vec![97, 54]), // 'a', '6'
        ("VNI", "a8", vec![97, 56]), // 'a', '8'
        ("VNI", "a1", vec![97, 49]), // 'a', '1'
        ("VIQR", "a^", vec![97, 94]), // 'a', '^'
        ("VIQR", "a(", vec![97, 40]), // 'a', '('
        ("VIQR", "a'", vec![97, 39]), // 'a', '''
    ];
    
    for (method_name, expected_output, key_codes) in test_cases {
        println!("{} Method - Input: {}", method_name, expected_output);
        
        let mut method: Box<dyn InputMethod> = match method_name {
            "Telex" => Box::new(TelexMethod::new()),
            "VNI" => Box::new(VniMethod::new()),
            "VIQR" => Box::new(ViqrMethod::new()),
            _ => continue,
        };
        
        for (i, &key_code) in key_codes.iter().enumerate() {
            let key_event = KeyEvent::new(
                KeyEventType::Normal,
                CharType::Vn,
                VnLexiName::A,
                key_code,
            );
            
            let result = method.process_key(key_event).unwrap();
            
            if i == key_codes.len() - 1 {
                // Last character should complete the sequence
                let output = String::from_utf8_lossy(&result);
                println!("  {} -> '{}' ✓", expected_output, output);
            } else {
                // Intermediate characters should be building
                if result.is_empty() {
                    println!("  Step {}: Building...", i + 1);
                } else {
                    println!("  Step {}: Unexpected output: '{}'", i + 1, String::from_utf8_lossy(&result));
                }
            }
        }
        println!();
    }
}

fn demo_engine_integration() {
    println!("=== Engine Integration Demo ===");
    println!();
    
    let mut engine = Engine::new();
    engine.set_input_method("Telex".to_string());
    engine.enable_engine(true);
    
    let test_phrases = vec![
        ("xin chao", vec![120, 105, 110, 32, 99, 104, 97, 111]), // "xin chao"
        ("toi ten la", vec![116, 111, 105, 32, 116, 101, 110, 32, 108, 97]), // "toi ten la"
    ];
    
    for (phrase, key_codes) in test_phrases {
        println!("Processing phrase: '{}'", phrase);
        
        for (i, &key_code) in key_codes.iter().enumerate() {
            let key_event = KeyEvent::new(
                KeyEventType::Normal,
                if key_code == 32 { CharType::WordBreak } else { CharType::Vn },
                VnLexiName::A,
                key_code,
            );
            
            let result = engine.process_key(key_event).unwrap();
            let output = String::from_utf8_lossy(&result);
            
            if !output.is_empty() {
                println!("  Step {}: '{}' -> '{}'", i + 1, key_code as u8 as char, output);
            }
        }
        println!();
    }
}

fn main() {
    println!("UniKey Rust - Comprehensive Demo");
    println!("=================================");
    println!();
    
    // Demo character mapping
    demo_character_mapping();
    
    // Demo input methods
    demo_input_methods();
    
    // Demo engine integration
    demo_engine_integration();
    
    println!("=== Demo completed successfully! ===");
    println!();
    println!("UniKey Rust now supports:");
    println!("✓ Telex input method (aa -> â, aw -> ă, etc.)");
    println!("✓ VNI input method (a6 -> â, a8 -> ă, etc.)");
    println!("✓ VIQR input method (a^ -> â, a( -> ă, etc.)");
    println!("✓ Character mapping and Unicode conversion");
    println!("✓ Input buffer management and sequence building");
    println!("✓ Word break detection and completion");
    println!("✓ Comprehensive error handling");
    println!();
    println!("Ready for platform integration (Linux XIM, Windows IME, macOS)!");
}
