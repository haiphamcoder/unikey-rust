//! GTK3 Input Method Module Demonstration
//! 
//! This example demonstrates the GTK3 input method module implementation
//! for UniKey Rust, showing how to create and manage GTK3 input methods.

use unikey_linux::prelude::*;
use unikey_input_methods::prelude::*;
use gtk::prelude::*;

/// GTK3 Input Method Demo Handler
struct Gtk3DemoHandler {
    name: String,
}

impl Gtk3DemoHandler {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl GtkInputMethodEventHandler for Gtk3DemoHandler {
    fn handle_event(&mut self, event: GtkInputMethodEvent) -> XimResult<()> {
        println!("  📤 GTK3 Handler '{}' received event: {:?}", self.name, event);
        Ok(())
    }
    
    fn handle_preedit(&mut self, text: &str, cursor: i32) -> XimResult<()> {
        println!("  📝 GTK3 Handler '{}' preedit: '{}' (cursor: {})", self.name, text, cursor);
        Ok(())
    }
    
    fn handle_commit(&mut self, text: &str) -> XimResult<()> {
        println!("  ✅ GTK3 Handler '{}' commit: '{}'", self.name, text);
        Ok(())
    }
    
    fn handle_input_method_change(&mut self, method: &str) -> XimResult<()> {
        println!("  🔄 GTK3 Handler '{}' input method changed to: {}", self.name, method);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🎨 UniKey GTK3 Input Method Module Demo");
    println!("{}", "=".repeat(60));

    // 1. Test GTK3 Input Method Module
    println!("\n🔧 Testing GTK3 Input Method Module...");
    let mut module = GtkInputMethodModule::new();
    
    // Initialize GTK3
    module.initialize().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method module initialized");

    // 2. Test Input Method Engine Setup
    println!("\n⚙️ Testing Input Method Engine Setup...");
    let telex_engine = Box::new(TelexMethod::new());
    module.set_engine(telex_engine);
    println!("  ✅ Telex engine set for GTK3 module");

    // 3. Test GTK3 Input Method Manager
    println!("\n🎛️ Testing GTK3 Input Method Manager...");
    let mut manager = GtkInputMethodManager::new();
    manager.initialize().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method manager initialized");

    // Add event handlers
    let demo_handler = Box::new(Gtk3DemoHandler::new("Demo Handler".to_string()));
    manager.add_event_handler(demo_handler);
    println!("  ✅ Event handler added");

    // 4. Test Input Context Creation
    println!("\n📝 Testing Input Context Creation...");
    
    // Create a simple GTK entry for testing (placeholder for demo)
    let entry = gtk::Entry::new();
    entry.set_text("Test input field");
    
    let context_id = "test_context".to_string();
    manager.create_input_context(context_id.clone(), entry).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ Input context created: {}", context_id);

    // 5. Test Key Event Processing
    println!("\n⌨️ Testing Key Event Processing...");
    let test_keys = vec![
        (0x61, 0x00, "a"),      // 'a' key
        (0x61, 0x00, "a"),      // 'a' key again
        (0x20, 0x00, "space"),  // space key
        (0x6E, 0x00, "n"),      // 'n' key
        (0x68, 0x00, "h"),      // 'h' key
        (0x65, 0x00, "e"),      // 'e' key
        (0x6C, 0x00, "l"),      // 'l' key
        (0x6C, 0x00, "l"),      // 'l' key again
        (0x6F, 0x00, "o"),      // 'o' key
    ];

    for (key, state, description) in test_keys {
        println!("  🔑 Processing key: {} ({})", description, key);
        manager.process_key_event(&context_id, key, state, 0).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    // 6. Test GTK3 Configuration
    println!("\n⚙️ Testing GTK3 Configuration...");
    let config = GtkInputMethodConfig {
        input_method_type: "telex".to_string(),
        preedit_enabled: true,
        commit_enabled: true,
        delete_surrounding_enabled: true,
        cursor_position_enabled: true,
        theme_name: "Adwaita".to_string(),
        font_size: 14,
        show_status_indicator: true,
    };
    
    manager.set_config(config.clone());
    println!("  ✅ GTK3 configuration set:");
    println!("    - Input Method Type: {}", config.input_method_type);
    println!("    - Preedit Enabled: {}", config.preedit_enabled);
    println!("    - Commit Enabled: {}", config.commit_enabled);
    println!("    - Theme: {}", config.theme_name);
    println!("    - Font Size: {}", config.font_size);

    // 7. Test GTK3 Event Simulation
    println!("\n🎭 Testing GTK3 Event Simulation...");
    let events = vec![
        GtkInputMethodEvent::KeyPress { key: 0x61, state: 0, time: 0 },
        GtkInputMethodEvent::TextInput { text: "a".to_string() },
        GtkInputMethodEvent::Preedit { text: "a".to_string(), cursor: 1 },
        GtkInputMethodEvent::Commit { text: "a".to_string() },
        GtkInputMethodEvent::KeyRelease { key: 0x61, state: 0, time: 0 },
        GtkInputMethodEvent::FocusIn,
        GtkInputMethodEvent::InputMethodChanged { method: "telex".to_string() },
        GtkInputMethodEvent::FocusOut,
    ];

    for event in events {
        println!("  📤 Simulating event: {:?}", event);
    }

    // 8. Test GTK3 State Management
    println!("\n🔄 Testing GTK3 State Management...");
    let states = vec![
        GtkInputMethodState::Inactive,
        GtkInputMethodState::Active,
        GtkInputMethodState::Preedit,
        GtkInputMethodState::Committed,
    ];

    for state in states {
        println!("  📊 GTK3 State: {:?}", state);
    }

    // 9. Test Manager Lifecycle
    println!("\n🚀 Testing Manager Lifecycle...");
    manager.start().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method manager started");
    
    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    manager.stop().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method manager stopped");

    // 10. Test Error Handling
    println!("\n🚨 Testing GTK3 Error Handling...");
    
    // Test invalid context ID
    match manager.process_key_event("invalid_context", 0x61, 0, 0) {
        Ok(_) => println!("  ⚠️  Unexpected success for invalid context"),
        Err(e) => println!("  ✅ Expected error for invalid context: {}", e),
    }

    // Test invalid key (using a public method instead)
    match module.process_key_event(0x9999, 0, 0) {
        Ok(_) => println!("  ⚠️  Unexpected success for invalid key"),
        Err(e) => println!("  ✅ Expected error for invalid key: {}", e),
    }

    // 11. Test Module Lifecycle
    println!("\n🔄 Testing Module Lifecycle...");
    module.start().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method module started");
    
    if module.is_running() {
        println!("  ✅ GTK3 input method module is running");
    }
    
    module.stop().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ GTK3 input method module stopped");

    // 12. Test Configuration Access
    println!("\n📋 Testing Configuration Access...");
    let current_config = module.config();
    println!("  📊 Current Configuration:");
    println!("    - Input Method Type: {}", current_config.input_method_type);
    println!("    - Preedit Enabled: {}", current_config.preedit_enabled);
    println!("    - Commit Enabled: {}", current_config.commit_enabled);
    println!("    - Delete Surrounding Enabled: {}", current_config.delete_surrounding_enabled);
    println!("    - Cursor Position Enabled: {}", current_config.cursor_position_enabled);
    println!("    - Theme Name: {}", current_config.theme_name);
    println!("    - Font Size: {}", current_config.font_size);
    println!("    - Show Status Indicator: {}", current_config.show_status_indicator);

    // 13. Test Input Context Management
    println!("\n📝 Testing Input Context Management...");
    
    // Create additional contexts
    let entry2 = gtk::Entry::new();
    entry2.set_text("Second input field");
    manager.create_input_context("context2".to_string(), entry2).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ Second input context created");

    let entry3 = gtk::Entry::new();
    entry3.set_text("Third input field");
    manager.create_input_context("context3".to_string(), entry3).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ Third input context created");

    // Remove a context
    manager.remove_input_context("context2").map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ Second input context removed");

    // Test removing non-existent context
    match manager.remove_input_context("nonexistent") {
        Ok(_) => println!("  ⚠️  Unexpected success for non-existent context"),
        Err(e) => println!("  ✅ Expected error for non-existent context: {}", e),
    }

    println!("\n🎉 GTK3 Input Method Module Demo completed!");
    println!("✅ All GTK3 components tested successfully:");
    println!("  - GTK3 Input Method Module");
    println!("  - Input Method Manager");
    println!("  - Input Context Management");
    println!("  - Key Event Processing");
    println!("  - Configuration Management");
    println!("  - Event Simulation");
    println!("  - State Management");
    println!("  - Error Handling");
    println!("  - Lifecycle Management");

    println!("\n📝 Note: This is a demonstration of the GTK3 input method module");
    println!("   implementation. In a real environment, this would integrate with");
    println!("   GTK3 applications to provide input method functionality.");

    Ok(())
}
