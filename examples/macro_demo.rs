//! Example demonstrating the macro system functionality

use unikey_macro::{MacroEngine, MacroDefinition};

fn main() {
    println!("🚀 UniKey Macro System Demo");
    println!("{}", "=".repeat(40));
    
    // Create a new macro engine
    let mut engine = MacroEngine::new();
    
    // Add some basic macros
    println!("\n📝 Adding basic macros...");
    
    let hello_macro = MacroDefinition::new(
        "hello".to_string(),
        "hello".to_string(),
        "Xin chào!".to_string(),
    );
    engine.add_macro(hello_macro).unwrap();
    
    let goodbye_macro = MacroDefinition::new(
        "goodbye".to_string(),
        "goodbye".to_string(),
        "Tạm biệt!".to_string(),
    );
    engine.add_macro(goodbye_macro).unwrap();
    
    let thanks_macro = MacroDefinition::new(
        "thanks".to_string(),
        "thanks".to_string(),
        "Cảm ơn!".to_string(),
    );
    engine.add_macro(thanks_macro).unwrap();
    
    // Add some Vietnamese-specific macros
    println!("📝 Adding Vietnamese-specific macros...");
    
    let vietnam_macro = MacroDefinition::new(
        "vietnam".to_string(),
        "vietnam".to_string(),
        "Việt Nam".to_string(),
    );
    engine.add_macro(vietnam_macro).unwrap();
    
    let hanoi_macro = MacroDefinition::new(
        "hanoi".to_string(),
        "hanoi".to_string(),
        "Hà Nội".to_string(),
    );
    engine.add_macro(hanoi_macro).unwrap();
    
    let saigon_macro = MacroDefinition::new(
        "saigon".to_string(),
        "saigon".to_string(),
        "Sài Gòn".to_string(),
    );
    engine.add_macro(saigon_macro).unwrap();
    
    // Add some wildcard macros
    println!("📝 Adding wildcard macros...");
    
    let email_macro = MacroDefinition::new(
        "email".to_string(),
        "*@*".to_string(),
        "[EMAIL]".to_string(),
    );
    engine.add_macro(email_macro).unwrap();
    
    let phone_macro = MacroDefinition::new(
        "phone".to_string(),
        "0*".to_string(),
        "[PHONE]".to_string(),
    );
    engine.add_macro(phone_macro).unwrap();
    
    // Test basic text processing
    println!("\n🧪 Testing basic text processing...");
    
    let test_texts = vec![
        "hello world",
        "goodbye everyone",
        "thanks for your help",
        "I love vietnam",
        "hanoi is beautiful",
        "saigon is busy",
        "Contact me at john@example.com",
        "Call me at 0123456789",
    ];
    
    for text in test_texts {
        let result = engine.process_text(text).unwrap();
        println!("  '{}' -> '{}'", text, result);
    }
    
    // Test case sensitivity
    println!("\n🧪 Testing case sensitivity...");
    
    let case_macro = MacroDefinition::new(
        "case_test".to_string(),
        "test".to_string(),
        "TEST".to_string(),
    ).with_case_sensitive(true);
    engine.add_macro(case_macro).unwrap();
    
    let case_texts = vec![
        "This is a test",
        "This is a TEST",
        "This is a Test",
    ];
    
    for text in case_texts {
        let result = engine.process_text(text).unwrap();
        println!("  '{}' -> '{}'", text, result);
    }
    
    // Test whole word matching
    println!("\n🧪 Testing whole word matching...");
    
    let word_macro = MacroDefinition::new(
        "word_test".to_string(),
        "test".to_string(),
        "EXAM".to_string(),
    ).with_whole_word(true);
    engine.add_macro(word_macro).unwrap();
    
    let word_texts = vec![
        "testing",
        "test case",
        "contest",
        "test",
    ];
    
    for text in word_texts {
        let result = engine.process_text(text).unwrap();
        println!("  '{}' -> '{}'", text, result);
    }
    
    // Show statistics
    println!("\n📊 Processing Statistics:");
    let stats = engine.get_stats();
    println!("  Total applications: {}", stats.total_applications);
    println!("  Successful applications: {}", stats.successful_applications);
    println!("  Failed applications: {}", stats.failed_applications);
    println!("  Total processing time: {}μs", stats.total_processing_time);
    
    // Show all macros
    println!("\n📋 All Macros:");
    for (id, macro_def) in engine.get_all_macros() {
        println!("  {}: '{}' -> '{}'", id, macro_def.pattern, macro_def.replacement);
    }
    
    println!("\n✅ Macro system demo completed successfully!");
}
