//! Character set definitions

/// Character set types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharsetType {
    Unicode,
    Utf8,
    Tcvn3,
    Vni,
    Viqr,
    Vps,
    Viscii,
    Bkhcm1,
    Bkhcm2,
    Vietwaref,
    Vietwarex,
    Vnimac,
    Isc,
    Wincp1258,
}

/// Character set converter trait
pub trait CharsetConverter {
    /// Convert from one charset to another
    fn convert(&self, input: &[u8], from: CharsetType, to: CharsetType) -> Result<Vec<u8>, String>;
}
