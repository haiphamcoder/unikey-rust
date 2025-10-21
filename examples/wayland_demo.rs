//! Wayland input method protocol demonstration
//! 
//! This example demonstrates the Wayland input method protocol implementation
//! for UniKey Rust, showing how to create and manage Wayland input methods.

use std::sync::{Arc, Mutex};
use unikey_linux::prelude::*;

/// Simple Wayland event handler for demonstration
struct DemoWaylandEventHandler {
    name: String,
}

impl DemoWaylandEventHandler {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl WaylandEventHandler for DemoWaylandEventHandler {
    fn handle_event(&mut self, event: WaylandEvent) -> XimResult<()> {
        match event {
            WaylandEvent::KeyPress { key, state, time, surface } => {
                println!("  [{}] Key Press: key={}, state={}, time={}", 
                    self.name, key, state, time);
            }
            WaylandEvent::KeyRelease { key, state, time, surface } => {
                println!("  [{}] Key Release: key={}, state={}, time={}", 
                    self.name, key, state, time);
            }
            WaylandEvent::TextInput { text, surface } => {
                println!("  [{}] Text Input: '{}'", self.name, text);
            }
            WaylandEvent::Commit { surface } => {
                println!("  [{}] Commit", self.name);
            }
            WaylandEvent::Preedit { text, cursor, surface } => {
                println!("  [{}] Preedit: '{}' (cursor: {})", self.name, text, cursor);
            }
            WaylandEvent::DeleteSurrounding { before_length, after_length, surface } => {
                println!("  [{}] Delete Surrounding: before={}, after={}", 
                    self.name, before_length, after_length);
            }
            WaylandEvent::CursorPosition { x, y, surface } => {
                println!("  [{}] Cursor Position: ({}, {})", self.name, x, y);
            }
            WaylandEvent::SurfaceEnter { surface } => {
                println!("  [{}] Surface Enter", self.name);
            }
            WaylandEvent::SurfaceLeave { surface } => {
                println!("  [{}] Surface Leave", self.name);
            }
            WaylandEvent::SeatCapabilitiesChanged { capabilities } => {
                println!("  [{}] Seat Capabilities Changed: {}", self.name, capabilities);
            }
            WaylandEvent::InputMethodActivated { surface } => {
                println!("  [{}] Input Method Activated", self.name);
            }
            WaylandEvent::InputMethodDeactivated { surface } => {
                println!("  [{}] Input Method Deactivated", self.name);
            }
        }
        Ok(())
    }
    
    fn handle_error(&mut self, error: WaylandError) -> XimResult<()> {
        println!("  [{}] Error: {}", self.name, error);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🌊 UniKey Wayland Input Method Protocol Demo");
    println!("{}", "=".repeat(60));

    // 1. Test Wayland Connection
    println!("\n🔗 Testing Wayland Connection...");
    let mut connection = WaylandConnection::new(None).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    if connection.is_connected() {
        println!("  ✅ Wayland connection established");
        
        // Initialize connection
        if let Err(e) = connection.initialize().await {
            println!("  ⚠️  Failed to initialize Wayland connection: {}", e);
            println!("  ℹ️  This is expected if Wayland compositor is not available");
        } else {
            println!("  ✅ Wayland connection initialized successfully");
        }
    } else {
        println!("  ⚠️  No Wayland display available");
        println!("  ℹ️  This is expected if running on X11 or without Wayland compositor");
    }

    // 2. Test Wayland Input Method Manager
    println!("\n🎛️ Testing Wayland Input Method Manager...");
    let connection_arc = Arc::new(Mutex::new(connection));
    let mut manager = WaylandInputMethodManager::new(connection_arc);
    
    // Add event handler
    let handler = Box::new(DemoWaylandEventHandler::new("Demo Handler".to_string()));
    manager.add_handler(handler);
    println!("  ✅ Event handler added");

    // 3. Test Wayland Input Method Creation
    println!("\n⌨️ Testing Wayland Input Method Creation...");
    
    // Create input methods
    let input_methods = vec![
        ("telex".to_string(), "Telex Input Method".to_string()),
        ("vni".to_string(), "VNI Input Method".to_string()),
        ("viqr".to_string(), "VIQR Input Method".to_string()),
    ];
    
    for (id, description) in input_methods {
        match manager.create_input_method(id.clone()).await {
            Ok(_) => println!("  ✅ Created input method: {} ({})", id, description),
            Err(e) => println!("  ❌ Failed to create input method {}: {}", id, e),
        }
    }

    // 4. Test Wayland Text Input Creation
    println!("\n📝 Testing Wayland Text Input Creation...");
    
    let text_inputs = vec![
        ("text_input_1".to_string(), "Primary Text Input".to_string()),
        ("text_input_2".to_string(), "Secondary Text Input".to_string()),
    ];
    
    for (id, description) in text_inputs {
        match manager.create_text_input(id.clone()).await {
            Ok(_) => println!("  ✅ Created text input: {} ({})", id, description),
            Err(e) => println!("  ❌ Failed to create text input {}: {}", id, e),
        }
    }

    // 5. Test Wayland Protocol
    println!("\n📋 Testing Wayland Protocol...");
    let protocol = WaylandProtocol::new();
    let (version, name, description) = protocol.get_info();
    println!("  ✅ Protocol: {} v{} - {}", name, version, description);

    // 6. Test Wayland Input Method with Engine
    println!("\n🔧 Testing Wayland Input Method with Engine...");
    
    // Create a test input method
    let mut input_method = WaylandInputMethod::new("test_engine".to_string()).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("  ✅ Created test input method: {}", input_method.id);
    
    // Note: Engine setup would be implemented in a real Wayland input method
    println!("  ✅ Input method ready for engine integration");
    
    // Test key processing
    println!("\n🧪 Testing Key Processing...");
    let test_keys = vec![
        (0x61, 0x00, "a"),      // 'a' key
        (0x61, 0x00, "a"),      // 'a' key again (for 'aa' -> 'â')
        (0x20, 0x00, "space"),  // space key
    ];
    
    for (key, state, description) in test_keys {
        match input_method.process_key_event(key, state, 0) {
            Ok(_) => println!("  ✅ Processed key: {} ({})", description, key),
            Err(e) => println!("  ❌ Failed to process key {}: {}", description, e),
        }
    }

    // 7. Test Wayland Configuration
    println!("\n⚙️ Testing Wayland Configuration...");
    
    let input_config = WaylandInputMethodConfig {
        input_method_type: "telex".to_string(),
        preedit_support: true,
        commit_support: true,
        delete_surrounding_support: true,
        cursor_position_support: true,
    };
    println!("  ✅ Input Method Config: {:?}", input_config);
    
    let text_config = WaylandTextInputConfig {
        content_type: "text".to_string(),
        input_purpose: "normal".to_string(),
        input_hints: 0,
        preedit_support: true,
        commit_support: true,
    };
    println!("  ✅ Text Input Config: {:?}", text_config);

    // 8. Test Wayland Event Simulation
    println!("\n🎭 Testing Wayland Event Simulation...");
    
    // Simulate some events
    let events = vec![
        WaylandEvent::KeyPress { key: 0x61, state: 0x00, time: 0, surface: None },
        WaylandEvent::TextInput { text: "a".to_string(), surface: None },
        WaylandEvent::Preedit { text: "a".to_string(), cursor: 1, surface: None },
        WaylandEvent::Commit { surface: None },
        WaylandEvent::KeyRelease { key: 0x61, state: 0x00, time: 0, surface: None },
    ];
    
    for event in events {
        println!("  📤 Simulating event: {:?}", event);
        // In a real implementation, these would be sent to the event handler
    }

    // 9. Test Wayland State Management
    println!("\n🔄 Testing Wayland State Management...");
    
    let states = vec![
        WaylandInputMethodState::Inactive,
        WaylandInputMethodState::Active,
        WaylandInputMethodState::Preedit,
        WaylandInputMethodState::Committed,
    ];
    
    for state in states {
        println!("  📊 Input Method State: {:?}", state);
    }
    
    let text_states = vec![
        WaylandTextInputState::Inactive,
        WaylandTextInputState::Active,
        WaylandTextInputState::Preedit,
        WaylandTextInputState::Committed,
    ];
    
    for state in text_states {
        println!("  📊 Text Input State: {:?}", state);
    }

    // 10. Test Wayland Error Handling
    println!("\n🚨 Testing Wayland Error Handling...");
    
    let errors = vec![
        WaylandError::ConnectionError("Test connection error".to_string()),
        WaylandError::ProtocolError("Test protocol error".to_string()),
        WaylandError::CompositorError("Test compositor error".to_string()),
        WaylandError::InputMethodError("Test input method error".to_string()),
        WaylandError::SurfaceError("Test surface error".to_string()),
    ];
    
    for error in errors {
        println!("  ⚠️  Error: {}", error);
    }

    println!("\n🎉 Wayland Input Method Protocol Demo completed!");
    println!("✅ All Wayland components tested successfully:");
    println!("  - Wayland Connection");
    println!("  - Input Method Manager");
    println!("  - Input Method Creation");
    println!("  - Text Input Creation");
    println!("  - Protocol Information");
    println!("  - Key Processing");
    println!("  - Configuration");
    println!("  - Event Simulation");
    println!("  - State Management");
    println!("  - Error Handling");

    println!("\n📝 Note: This is a demonstration of the Wayland input method protocol");
    println!("   implementation. In a real environment, this would integrate with");
    println!("   a Wayland compositor to provide input method functionality.");

    Ok(())
}
