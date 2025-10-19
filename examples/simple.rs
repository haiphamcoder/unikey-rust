//! Simple example of using UniKey

use unikey_core::{Engine, KeyEvent, KeyEventType, CharType, VnLexiName};

fn main() {
    println!("UniKey Rust - Simple Example");
    
    // Create a new UniKey engine
    let mut engine = Engine::new();
    
    // Set input method to Telex
    engine.set_input_method("Telex".to_string());
    
    // Set output type to Unicode
    engine.set_output_type("Unicode".to_string());
    
    // Enable the engine
    engine.enable_engine(true);
    
    println!("Engine created and configured");
    println!("Input method: {}", engine.get_input_method());
    println!("Output type: {}", engine.get_output_type());
    println!("Engine enabled: {}", engine.is_engine_enabled());
    
    // Create a sample key event
    let key_event = KeyEvent::new(
        KeyEventType::Normal,
        CharType::Vn,
        VnLexiName::A,
        65, // 'A' key
    );
    
    println!("Sample key event: {:?}", key_event);
    
    // Process the key event
    match engine.process_key(key_event) {
        Ok(output) => {
            println!("Processed output: {:?}", output);
        }
        Err(e) => {
            println!("Error processing key: {}", e);
        }
    }
    
    // Test some Vietnamese character input
    println!("\nTesting Vietnamese character input:");
    
    // Test 'a' -> 'a'
    let mut engine2 = Engine::new();
    engine2.set_input_method("Telex".to_string());
    engine2.enable_engine(true);
    
    let key_a = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97); // 'a'
    match engine2.process_key(key_a) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("Input 'a' -> Output: '{}'", result);
        }
        Err(e) => {
            println!("Error processing 'a': {}", e);
        }
    }
    
    // Test 'aa' -> 'â'
    let mut engine3 = Engine::new();
    engine3.set_input_method("Telex".to_string());
    engine3.enable_engine(true);
    
    // First 'a'
    let key_a1 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    let _ = engine3.process_key(key_a1);
    
    // Second 'a' to complete 'aa' -> 'â'
    let key_a2 = KeyEvent::new(KeyEventType::Normal, CharType::Vn, VnLexiName::A, 97);
    match engine3.process_key(key_a2) {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output);
            println!("Input 'aa' -> Output: '{}'", result);
        }
        Err(e) => {
            println!("Error processing 'aa': {}", e);
        }
    }
    
    println!("\nSimple example completed successfully!");
}