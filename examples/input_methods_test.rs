//! Test all input methods (Telex, VNI, VIQR)

use unikey_input_methods::{TelexMethod, VniMethod, ViqrMethod, InputMethod};
use unikey_core::{KeyEvent, KeyEventType, CharType, VnLexiName};

fn test_input_method<M: InputMethod>(name: &str, mut method: M) {
    println!("\n=== Testing {} Input Method ===", name);
    
    // Test basic characters
    println!("\n1. Basic character sequences:");
    
    // Test 'a' -> wait
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let result = method.process_key(key_a).unwrap();
    println!("  'a' -> '{}' (buffer: '{}')", 
             String::from_utf8_lossy(&result), 
             if result.is_empty() { "building..." } else { "completed" });
    
    // Test 'aa' -> 'â' (Telex), 'a6' -> 'â' (VNI), 'a^' -> 'â' (VIQR)
    let key_second = match name {
        "Telex" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97), // 'a'
        "VNI" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 54),   // '6'
        "VIQR" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 94),  // '^'
        _ => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97),
    };
    
    let result = method.process_key(key_second).unwrap();
    println!("  '{}' -> '{}'", 
             match name {
                 "Telex" => "aa",
                 "VNI" => "a6", 
                 "VIQR" => "a^",
                 _ => "??",
             },
             String::from_utf8_lossy(&result));
    
    // Test diacritics
    println!("\n2. Diacritic characters:");
    
    // Reset method
    method.reset();
    
    // Test 'a' + diacritic
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = method.process_key(key_a);
    
    let key_diacritic = match name {
        "Telex" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 119), // 'w' for 'aw' -> 'ă'
        "VNI" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 56),    // '8' for 'a8' -> 'ă'
        "VIQR" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 40),   // '(' for 'a(' -> 'ă'
        _ => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97),
    };
    
    let result = method.process_key(key_diacritic).unwrap();
    println!("  '{}' -> '{}'", 
             match name {
                 "Telex" => "aw",
                 "VNI" => "a8",
                 "VIQR" => "a(",
                 _ => "??",
             },
             String::from_utf8_lossy(&result));
    
    // Test tone marks
    println!("\n3. Tone marks:");
    
    // Reset method
    method.reset();
    
    // Test 'a' + tone
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = method.process_key(key_a);
    
    let key_tone = match name {
        "Telex" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 49),  // '1' for 'a1' -> 'á'
        "VNI" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 49),    // '1' for 'a1' -> 'á'
        "VIQR" => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 39),   // ''' for 'a\'' -> 'á'
        _ => KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 49),
    };
    
    let result = method.process_key(key_tone).unwrap();
    println!("  '{}' -> '{}'", 
             match name {
                 "Telex" => "a1",
                 "VNI" => "a1",
                 "VIQR" => "a'",
                 _ => "??",
             },
             String::from_utf8_lossy(&result));
    
    // Test word break
    println!("\n4. Word break handling:");
    
    // Reset method
    method.reset();
    
    // Test 'a' + space
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = method.process_key(key_a);
    
    let key_space = KeyEvent::new(KeyEventType::Normal, CharType::WordBreak, VnLexiName::A, 32);
    let result = method.process_key(key_space).unwrap();
    println!("  'a' + space -> '{}'", String::from_utf8_lossy(&result));
}

fn main() {
    println!("UniKey Input Methods Test");
    println!("=========================");
    
    // Test Telex method
    test_input_method("Telex", TelexMethod::new());
    
    // Test VNI method
    test_input_method("VNI", VniMethod::new());
    
    // Test VIQR method
    test_input_method("VIQR", ViqrMethod::new());
    
    println!("\n=== All tests completed ===");
}
