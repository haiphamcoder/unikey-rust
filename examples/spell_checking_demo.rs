//! Example demonstrating the spell checking and smart features functionality

use unikey_spell::{SpellChecker, WordType, ProcessingMode};

fn main() {
    println!("🔍 UniKey Spell Checking & Smart Features Demo");
    println!("{}", "=".repeat(50));
    
    // Create a new spell checker
    let mut checker = SpellChecker::new().unwrap();
    
    // Test word type detection
    println!("\n📝 Testing Word Type Detection:");
    test_word_detection(&checker);
    
    // Test spell checking
    println!("\n✅ Testing Spell Checking:");
    test_spell_checking(&checker);
    
    // Test suggestions
    println!("\n💡 Testing Suggestions:");
    test_suggestions(&checker);
    
    // Test smart processing
    println!("\n🧠 Testing Smart Processing:");
    test_smart_processing(&mut checker);
    
    // Test free marking
    println!("\n🎯 Testing Free Marking:");
    test_free_marking(&mut checker);
    
    // Show statistics
    println!("\n📊 Spell Checker Statistics:");
    let stats = checker.get_stats();
    println!("  Vietnamese words: {}", stats.vietnamese_word_count);
    println!("  English words: {}", stats.english_word_count);
    println!("  Free marking patterns: {}", stats.free_marking_pattern_count);
    println!("  Total words: {}", stats.total_word_count);
    
    println!("\n✅ Spell checking demo completed successfully!");
}

fn test_word_detection(checker: &SpellChecker) {
    let test_words = vec![
        ("xin chào", WordType::Vietnamese),
        ("hello", WordType::English),
        ("123", WordType::Number),
        ("!", WordType::Punctuation),
        ("xin", WordType::VietnamesePlain),
        ("hello123", WordType::Mixed),
    ];
    
    for (word, expected_type) in test_words {
        let detected_type = checker.detect_word_type(word);
        let status = if detected_type == expected_type { "✅" } else { "❌" };
        println!("  {} '{}' -> {:?} (expected: {:?})", status, word, detected_type, expected_type);
    }
}

fn test_spell_checking(checker: &SpellChecker) {
    let test_words = vec![
        ("xin", true),      // Vietnamese word
        ("chào", true),     // Vietnamese word
        ("hello", true),    // English word
        ("xyz", false),     // Unknown word
        ("123", true),      // Number
        ("!", true),        // Punctuation
    ];
    
    for (word, expected_correct) in test_words {
        let is_correct = checker.is_spelled_correctly(word);
        let status = if is_correct == expected_correct { "✅" } else { "❌" };
        println!("  {} '{}' -> {} (expected: {})", status, word, is_correct, expected_correct);
    }
}

fn test_suggestions(checker: &SpellChecker) {
    let test_words = vec![
        "xin",      // Should suggest "xin" (already correct)
        "chao",     // Should suggest "chào"
        "cam",      // Should suggest "cảm"
        "on",       // Should suggest "ơn"
        "xyz",      // Should suggest nothing (unknown)
    ];
    
    for word in test_words {
        let suggestions = checker.get_suggestions(word);
        if suggestions.is_empty() {
            println!("  '{}' -> No suggestions", word);
        } else {
            println!("  '{}' -> Suggestions: {:?}", word, suggestions);
        }
    }
}

fn test_smart_processing(checker: &mut SpellChecker) {
    let test_texts = vec![
        "xin chào thế giới",
        "hello world",
        "xin chào hello world",
        "tôi là học sinh",
        "I am a student",
    ];
    
    for text in test_texts {
        let result = checker.process_text(text, ProcessingMode::Smart).unwrap();
        println!("  '{}' -> '{}'", text, result.processed);
        
        if !result.suggestions.is_empty() {
            println!("    Suggestions:");
            for suggestion in result.suggestions {
                println!("      '{}' -> '{}' (confidence: {:.2})", 
                    suggestion.original, suggestion.suggestion, suggestion.confidence);
            }
        }
    }
}

fn test_free_marking(checker: &mut SpellChecker) {
    // Add some free marking patterns
    checker.add_free_marking_pattern("btw".to_string(), "by the way".to_string());
    checker.add_free_marking_pattern("lol".to_string(), "laugh out loud".to_string());
    checker.add_free_marking_pattern("ko".to_string(), "không".to_string());
    
    let test_texts = vec![
        "btw hello world",
        "lol that's funny",
        "ko tôi ko biết",
        "xin chào btw",
    ];
    
    for text in test_texts {
        let result = checker.process_text(text, ProcessingMode::FreeMarking).unwrap();
        println!("  '{}' -> '{}'", text, result.processed);
    }
}
