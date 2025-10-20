//! Example demonstrating Linux XIM integration

use unikey_linux::{XimServer, XimConfig};
use unikey_input_methods::InputMethodType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 UniKey Linux XIM Integration Demo");
    println!("{}", "=".repeat(50));

    // Create XIM configuration
    let mut config = XimConfig::new("unikey-rust-demo".to_string());
    config.display_name = Some(":0".to_string());
    config.default_input_method = InputMethodType::Telex;
    config.available_input_methods = vec![
        InputMethodType::Telex,
        InputMethodType::VNI,
        InputMethodType::VIQR,
    ];
    config.ui.show_status_indicator = true;
    config.logging.level = unikey_linux::LogLevel::Info;
    config.advanced.debug_mode = true;

    println!("📋 XIM Configuration:");
    println!("  Server Name: {}", config.server_name);
    println!("  Display Name: {:?}", config.display_name);
    println!("  Default Input Method: {:?}", config.default_input_method);
    println!("  Available Input Methods: {:?}", config.available_input_methods);
    println!("  Show Status Indicator: {}", config.ui.show_status_indicator);
    println!("  Log Level: {:?}", config.logging.level);
    println!("  Debug Mode: {}", config.advanced.debug_mode);

    // Create XIM server
    println!("\n🔧 Creating XIM server...");
    let server = XimServer::new(config).await?;
    println!("✅ XIM server created successfully");

    // Show server information
    println!("\n📊 Server Information:");
    println!("  Client Count: {}", server.get_client_count().await);
    println!("  Running: {}", server.is_running().await);

    // Start server (in a separate task for demo)
    println!("\n🚀 Starting XIM server...");
    let server_handle = {
        let mut server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.run().await {
                eprintln!("❌ Server error: {}", e);
            }
        })
    };

    // Let server run for a few seconds
    println!("⏳ Server running for 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Stop server
    println!("\n🛑 Stopping XIM server...");
    server.stop().await?;
    server_handle.abort();

    println!("✅ XIM server stopped successfully");
    println!("\n🎉 Linux XIM integration demo completed!");

    Ok(())
}
