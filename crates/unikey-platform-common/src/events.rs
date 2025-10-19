//! Platform events

/// Platform event
#[derive(Debug, Clone)]
pub enum PlatformEvent {
    KeyPress { key: u32, modifiers: u32 },
    KeyRelease { key: u32, modifiers: u32 },
    FocusIn,
    FocusOut,
    Quit,
}
