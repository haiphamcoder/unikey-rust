//! Simple example of using UniKey

use unikey_core::{UnikeyEngine, KeyEvent, KeyEventType, CharType, VnLexiName, InputMethod, OutputType};

fn main() {
    println!("UniKey Rust - Simple Example");
    
    // Create a new UniKey engine
    let mut engine = UnikeyEngine::new();
    
    // Set input method to Telex
    engine.set_input_method(InputMethod::Telex);
    
    // Set output type to Unicode
    engine.set_output_type(OutputType::Unicode);
    
    // Enable the engine
    engine.set_enabled(true);
    
    println!("Engine created and configured");
    println!("Input method: {:?}", InputMethod::Telex);
    println!("Output type: {:?}", OutputType::Unicode);
    println!("Engine enabled: {}", engine.is_enabled());
    
    // Create a sample key event
    let key_event = KeyEvent {
        event_type: KeyEventType::Normal,
        char_type: CharType::Vn,
        vn_sym: VnLexiName::A,
        key_code: 65, // 'A' key
        tone: 0,
    };
    
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
}
