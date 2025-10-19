//! Debug test for Vietnamese character processing

use unikey_core::{Engine, KeyEvent, KeyEventType, CharType, VnLexiName, CharMapper};

fn main() {
    println!("UniKey Rust - Debug Test");
    println!("========================");
    
    // Test character mapping directly
    println!("\n1. Testing character mapping directly:");
    println!("'aa' -> Unicode: {:?}", CharMapper::telex_to_unicode("aa"));
    println!("'aw' -> Unicode: {:?}", CharMapper::telex_to_unicode("aw"));
    println!("'ee' -> Unicode: {:?}", CharMapper::telex_to_unicode("ee"));
    println!("'oo' -> Unicode: {:?}", CharMapper::telex_to_unicode("oo"));
    println!("'ow' -> Unicode: {:?}", CharMapper::telex_to_unicode("ow"));
    println!("'uw' -> Unicode: {:?}", CharMapper::telex_to_unicode("uw"));
    
    // Test engine with single character
    println!("\n2. Testing engine with single character 'a':");
    let mut engine = Engine::new();
    engine.set_input_method("Telex".to_string());
    engine.enable_engine(true);
    
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    match engine.process_key(key_a) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("Input 'a' -> Output: '{}' (bytes: {:?})", result, output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Test engine with 'aa' sequence
    println!("\n3. Testing engine with 'aa' sequence:");
    let mut engine2 = Engine::new();
    engine2.set_input_method("Telex".to_string());
    engine2.enable_engine(true);
    
    // First 'a'
    let key_a1 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    println!("Processing first 'a'...");
    match engine2.process_key(key_a1) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("  First 'a' -> Output: '{}' (bytes: {:?})", result, output);
            println!("  Buffer: '{}'", engine2.get_buffer());
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
    
    // Second 'a'
    let key_a2 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    println!("Processing second 'a'...");
    match engine2.process_key(key_a2) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("  Second 'a' -> Output: '{}' (bytes: {:?})", result, output);
            println!("  Buffer: '{}'", engine2.get_buffer());
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
    
    // Test with word break to force completion
    println!("\n4. Testing with word break to force completion:");
    let mut engine3 = Engine::new();
    engine3.set_input_method("Telex".to_string());
    engine3.enable_engine(true);
    
    // First 'a'
    let key_a1 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = engine3.process_key(key_a1);
    println!("  After first 'a', buffer: '{}'", engine3.get_buffer());
    
    // Second 'a'
    let key_a2 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = engine3.process_key(key_a2);
    println!("  After second 'a', buffer: '{}'", engine3.get_buffer());
    
    // Word break
    let key_space = KeyEvent::new(KeyEventType::Normal, CharType::WordBreak, VnLexiName::A, 32);
    match engine3.process_key(key_space) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("  After space -> Output: '{}' (bytes: {:?})", result, output);
            println!("  Buffer: '{}'", engine3.get_buffer());
        }
        Err(e) => {
            println!("  Error: {}", e);
        }
    }
    
    println!("\nDebug test completed!");
}
