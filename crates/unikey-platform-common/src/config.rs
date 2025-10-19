//! Platform configuration

use serde::{Deserialize, Serialize};

/// Platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub input_method: String,
    pub output_type: String,
    pub enabled: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            input_method: "telex".to_string(),
            output_type: "unicode".to_string(),
            enabled: true,
        }
    }
}
