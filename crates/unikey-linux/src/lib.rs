//! # UniKey Linux
//! 
//! Linux platform integration for UniKey.
//! 
//! This crate provides Linux-specific implementations including XIM server,
//! Wayland input method protocol, and GTK integration.

pub mod xim;
pub mod wayland;
pub mod gtk;
pub mod tray;
pub mod error;

pub use xim::*;
pub use wayland::*;
pub use gtk::*;
pub use tray::*;
pub use error::*;

/// Linux platform functionality re-exports
pub mod prelude {
    pub use crate::{
        xim::*,
        wayland::*,
        gtk::*,
        tray::*,
        error::*,
    };
}
