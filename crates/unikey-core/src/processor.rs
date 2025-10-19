//! Input processor for UniKey

use crate::{KeyEvent, CharType, VnLexiName, Result};

/// Input processor trait
pub trait InputProcessor {
    /// Process a key event
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>>;
    
    /// Reset processor state
    fn reset(&mut self);
}

/// Basic input processor
pub struct BasicProcessor {
    buffer: Vec<u8>,
}

impl BasicProcessor {
    /// Create a new basic processor
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}

impl InputProcessor for BasicProcessor {
    fn process_key(&mut self, event: KeyEvent) -> Result<Vec<u8>> {
        // TODO: Implement basic key processing
        Ok(vec![])
    }
    
    fn reset(&mut self) {
        self.buffer.clear();
    }
}

impl Default for BasicProcessor {
    fn default() -> Self {
        Self::new()
    }
}
