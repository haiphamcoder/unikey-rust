//! System tray implementation for Linux using GTK3

use crate::{XimError, XimResult};
use gtk::prelude::*;
use gtk::{Application, Menu, MenuItem};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// System tray menu actions
#[derive(Debug, Clone)]
pub enum TrayAction {
    ToggleInputMethod,
    SwitchInputMethod,
    ShowConfiguration,
    ShowAbout,
    Quit,
}

/// System tray event handler trait
pub trait TrayEventHandler: Send + Sync {
    fn handle_action(&mut self, action: TrayAction) -> XimResult<()>;
}

/// System tray implementation
pub struct SystemTray {
    app: Application,
    menu: Menu,
    handlers: Vec<Box<dyn TrayEventHandler>>,
    running: Arc<AtomicBool>,
}

impl SystemTray {
    /// Create a new system tray
    pub fn new(app_id: &str) -> XimResult<Self> {
        // Initialize GTK
        gtk::init().map_err(|e| XimError::X11Error(format!("Failed to initialize GTK: {}", e)))?;
        
        // Create GTK application
        let app = Application::new(Some(app_id), gio::ApplicationFlags::empty());
        
        // Create menu
        let menu = Menu::new();
        
        Ok(Self {
            app,
            menu,
            handlers: Vec::new(),
            running: Arc::new(AtomicBool::new(false)),
        })
    }
    
    /// Add event handler
    pub fn add_handler(&mut self, handler: Box<dyn TrayEventHandler>) {
        self.handlers.push(handler);
    }
    
    /// Show tray icon
    pub fn show(&mut self) -> XimResult<()> {
        // Create menu items
        self.create_menu_items();
        
        self.running.store(true, Ordering::SeqCst);
        
        log::info!("System tray icon shown (simplified)");
        Ok(())
    }
    
    /// Hide tray icon
    pub fn hide(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        
        log::info!("System tray icon hidden");
    }
    
    /// Update tray icon
    pub fn update_icon(&mut self, icon_name: &str, tooltip: &str) {
        log::debug!("Updating tray icon: {} - {}", icon_name, tooltip);
    }
    
    /// Run the GTK main loop
    pub async fn run(&self) -> XimResult<()> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(XimError::X11Error("Tray not started".to_string()));
        }
        
        // Run GTK main loop in a separate thread
        let running = self.running.clone();
        tokio::task::spawn_blocking(move || {
            while running.load(Ordering::SeqCst) {
                gtk::main_iteration();
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }).await.map_err(|e| XimError::X11Error(format!("GTK main loop error: {}", e)))?;
        
        Ok(())
    }
    
    /// Create menu items
    fn create_menu_items(&mut self) {
        // Toggle Input Method
        let toggle_item = MenuItem::with_label("Toggle Input Method");
        toggle_item.connect_activate(move |_| {
            log::info!("Toggle Input Method clicked");
        });
        self.menu.append(&toggle_item);
        
        // Switch Input Method
        let switch_item = MenuItem::with_label("Switch Input Method");
        switch_item.connect_activate(move |_| {
            log::info!("Switch Input Method clicked");
        });
        self.menu.append(&switch_item);
        
        // Separator
        let separator = gtk::SeparatorMenuItem::new();
        self.menu.append(&separator);
        
        // Configuration
        let config_item = MenuItem::with_label("Configuration");
        config_item.connect_activate(move |_| {
            log::info!("Configuration clicked");
        });
        self.menu.append(&config_item);
        
        // About
        let about_item = MenuItem::with_label("About");
        about_item.connect_activate(move |_| {
            log::info!("About clicked");
        });
        self.menu.append(&about_item);
        
        // Separator
        let separator2 = gtk::SeparatorMenuItem::new();
        self.menu.append(&separator2);
        
        // Quit
        let quit_item = MenuItem::with_label("Quit");
        quit_item.connect_activate(move |_| {
            log::info!("Quit clicked");
        });
        self.menu.append(&quit_item);
        
        // Show all menu items
        self.menu.show_all();
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}
