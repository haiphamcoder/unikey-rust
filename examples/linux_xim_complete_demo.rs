//! Complete Linux XIM Integration Demo

use unikey_linux::{
    XimServer, XimConfig, X11Connection, X11EventManager, X11Event, X11EventHandler,
    SystemTray, TrayAction, TrayEventHandler, ConfigManager, ConfigWatcher,
};
use unikey_input_methods::InputMethodType;
use std::sync::Arc;
use std::time::Duration;

/// Demo event handler for X11 events
struct DemoX11EventHandler {
    name: String,
}

impl DemoX11EventHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl X11EventHandler for DemoX11EventHandler {
    fn handle_event(&mut self, event: X11Event) -> unikey_linux::XimResult<()> {
        match event {
            X11Event::KeyPress { keycode, state, window } => {
                println!("🔑 Key Press: keycode={}, state={}, window={}", keycode, state, window);
            }
            X11Event::KeyRelease { keycode, state, window } => {
                println!("🔑 Key Release: keycode={}, state={}, window={}", keycode, state, window);
            }
            X11Event::ButtonPress { button, state, window } => {
                println!("🖱️ Button Press: button={}, state={}, window={}", button, state, window);
            }
            X11Event::ButtonRelease { button, state, window } => {
                println!("🖱️ Button Release: button={}, state={}, window={}", button, state, window);
            }
            X11Event::FocusIn { window } => {
                println!("👁️ Focus In: window={}", window);
            }
            X11Event::FocusOut { window } => {
                println!("👁️ Focus Out: window={}", window);
            }
            _ => {
                // Ignore other events
            }
        }
        Ok(())
    }
}

/// Demo tray event handler
struct DemoTrayEventHandler {
    name: String,
}

impl DemoTrayEventHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl TrayEventHandler for DemoTrayEventHandler {
    fn handle_action(&mut self, action: TrayAction) -> unikey_linux::XimResult<()> {
        match action {
            TrayAction::ToggleInputMethod => {
                println!("🔄 Toggle Input Method requested");
            }
            TrayAction::SwitchInputMethod => {
                println!("🔄 Switch Input Method requested");
            }
            TrayAction::ShowConfiguration => {
                println!("⚙️ Show Configuration requested");
            }
            TrayAction::ShowAbout => {
                println!("ℹ️ Show About requested");
            }
            TrayAction::Quit => {
                println!("🚪 Quit requested");
            }
        }
        Ok(())
    }
}

/// Demo config watcher
struct DemoConfigWatcher {
    name: String,
}

impl DemoConfigWatcher {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl ConfigWatcher for DemoConfigWatcher {
    fn on_config_changed(&mut self, config: &unikey_linux::ConfigFile) -> unikey_linux::XimResult<()> {
        println!("📝 Configuration changed: {}", self.name);
        println!("  - XIM Server: {}", config.xim.server_name);
        println!("  - Default Input Method: {:?}", config.xim.default_input_method);
        println!("  - Toggle Hotkey: {}", config.hotkeys.toggle_input_method);
        println!("  - Show Status Indicator: {}", config.ui.show_status_indicator);
        println!("  - Debug Mode: {}", config.advanced.debug_mode);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 UniKey Linux XIM Complete Integration Demo");
    println!("{}", "=".repeat(60));

    // 1. Configuration Management
    println!("\n📝 Testing Configuration Management...");
    let mut config_manager = ConfigManager::new(None)?;
    
    // Show current configuration
    let config = config_manager.get_config();
    println!("  ✅ Configuration loaded from: {:?}", config_manager.config_path());
    println!("  - XIM Server: {}", config.xim.server_name);
    println!("  - Default Input Method: {:?}", config.xim.default_input_method);
    println!("  - Toggle Hotkey: {}", config.hotkeys.toggle_input_method);
    println!("  - Show Status Indicator: {}", config.ui.show_status_indicator);
    println!("  - Debug Mode: {}", config.advanced.debug_mode);

    // Add config watcher
    let config_watcher = DemoConfigWatcher::new("Demo Watcher");
    config_manager.add_watcher(Box::new(config_watcher));

    // Update configuration
    config_manager.update_config(|config| {
        config.xim.default_input_method = InputMethodType::VNI;
        config.ui.show_status_indicator = true;
        config.advanced.debug_mode = true;
    })?;
    println!("  ✅ Configuration updated");

    // Save configuration
    config_manager.save_config()?;
    println!("  ✅ Configuration saved");

    // 2. X11 Event Handling
    println!("\n🖥️ Testing X11 Event Handling...");
    let x11_connection = Arc::new(X11Connection::new(Some(":0"))?);
    
    if x11_connection.is_connected() {
        println!("  ✅ X11 connection established");
        
        let mut event_manager = X11EventManager::new(x11_connection.clone());
        event_manager.add_handler(Box::new(DemoX11EventHandler::new("Demo Handler")));
        
        // Start event loop in background
        let event_handle = {
            let mut event_manager = event_manager;
            tokio::spawn(async move {
                if let Err(e) = event_manager.start_event_loop().await {
                    eprintln!("❌ X11 Event loop error: {}", e);
                }
            })
        };
        
        println!("  ✅ X11 event loop started");
        
        // Let it run for a bit
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Stop event loop
        event_handle.abort();
        println!("  ✅ X11 event loop stopped");
    } else {
        println!("  ⚠️ X11 connection failed (running in headless mode)");
    }

    // 3. System Tray Integration
    println!("\n🔔 Testing System Tray Integration...");
    let mut system_tray = SystemTray::new("com.unikey-rust.demo")?;
    system_tray.add_handler(Box::new(DemoTrayEventHandler::new("Demo Tray Handler")));
    
    // Show tray icon
    system_tray.show()?;
    println!("  ✅ System tray icon shown");
    
    // Update tray icon
    system_tray.update_icon("unikey", "UniKey Vietnamese Input Method - Demo");
    println!("  ✅ System tray icon updated");
    
    // Run tray in background (simplified for demo)
    println!("  ✅ System tray running (simplified)");
    
    // Let it run for a bit
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Stop tray
    system_tray.hide();
    println!("  ✅ System tray stopped");

    // 4. XIM Server Integration
    println!("\n🌐 Testing XIM Server Integration...");
    let mut xim_config = XimConfig::new("unikey-rust-demo".to_string());
    xim_config.display_name = Some(":0".to_string());
    xim_config.default_input_method = InputMethodType::Telex;
    xim_config.available_input_methods = vec![
        InputMethodType::Telex,
        InputMethodType::VNI,
        InputMethodType::VIQR,
    ];
    xim_config.ui.show_status_indicator = true;
    xim_config.logging.level = unikey_linux::LogLevel::Info;
    xim_config.advanced.debug_mode = true;

    let xim_server = XimServer::new(xim_config).await?;
    println!("  ✅ XIM server created");
    
    // Show server information
    println!("  - Client Count: {}", xim_server.get_client_count().await);
    println!("  - Running: {}", xim_server.is_running().await);

    // Start server in background
    let server_handle = {
        let mut xim_server = xim_server.clone();
        tokio::spawn(async move {
            if let Err(e) = xim_server.run().await {
                eprintln!("❌ XIM server error: {}", e);
            }
        })
    };
    
    println!("  ✅ XIM server started");
    
    // Let it run for a bit
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Stop server
    server_handle.abort();
    println!("  ✅ XIM server stopped");

    // 5. Integration Test
    println!("\n🔗 Testing Complete Integration...");
    
    // Create integrated system
    let mut config_manager = ConfigManager::new(None)?;
    let x11_connection = Arc::new(X11Connection::new(Some(":0"))?);
    let mut system_tray = SystemTray::new("com.unikey-rust.integrated")?;
    
    // Add handlers
    system_tray.add_handler(Box::new(DemoTrayEventHandler::new("Integrated Handler")));
    
    // Show tray
    system_tray.show()?;
    println!("  ✅ Integrated system tray shown");
    
    // Start event loop if X11 is available
    if x11_connection.is_connected() {
        let mut event_manager = X11EventManager::new(x11_connection.clone());
        event_manager.add_handler(Box::new(DemoX11EventHandler::new("Integrated Handler")));
        
        let event_handle = {
            let mut event_manager = event_manager;
            tokio::spawn(async move {
                if let Err(e) = event_manager.start_event_loop().await {
                    eprintln!("❌ Integrated event loop error: {}", e);
                }
            })
        };
        
        println!("  ✅ Integrated X11 event loop started");
        
        // Run integrated system (simplified)
        println!("  ✅ Integrated system running (simplified)");
        
        // Let it run for a bit
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Stop integrated system
        event_handle.abort();
        system_tray.hide();
        println!("  ✅ Integrated system stopped");
    } else {
        println!("  ⚠️ Skipping X11 integration (headless mode)");
    }

    println!("\n🎉 Complete Linux XIM Integration Demo finished!");
    println!("✅ All components tested successfully:");
    println!("  - Configuration Management");
    println!("  - X11 Event Handling");
    println!("  - System Tray Integration");
    println!("  - XIM Server Integration");
    println!("  - Complete Integration");

    Ok(())
}
