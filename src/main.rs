//! UniKey Rust - Main binary

use unikey_core::{Engine, KeyEvent, KeyEventType, CharType, VnLexiName};

fn main() {
    println!("UniKey Rust - Vietnamese Input Method");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    
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
    
    println!("UniKey Rust example completed successfully!");
}