//! Core data types for UniKey

/// Vietnamese lexical names - based on original UniKey implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum VnLexiName {
    NonVnChar = -1,
    // Base characters
    A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h,
    I, i, J, j, K, k, L, l, M, m, N, n, O, o, P, p,
    Q, q, R, r, S, s, T, t, U, u, V, v, W, w, X, x,
    Y, y, Z, z,
    
    // Vietnamese specific characters
    DD, dd,  // đ, Đ
    
    // Vowels with tone marks (0-5)
    A0, a0, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5,
    E0, e0, E1, e1, E2, e2, E3, e3, E4, e4, E5, e5,
    I0, i0, I1, i1, I2, i2, I3, i3, I4, i4, I5, i5,
    O0, o0, O1, o1, O2, o2, O3, o3, O4, o4, O5, o5,
    U0, u0, U1, u1, U2, u2, U3, u3, U4, u4, U5, u5,
    Y0, y0, Y1, y1, Y2, y2, Y3, y3, Y4, y4, Y5, y5,
    
    // Vowels with diacritics
    Ar, ar, Ar0, ar0, Ar1, ar1, Ar2, ar2, Ar3, ar3, Ar4, ar4, Ar5, ar5,  // ă
    Ab, ab, Ab0, ab0, Ab1, ab1, Ab2, ab2, Ab3, ab3, Ab4, ab4, Ab5, ab5,  // â
    Er, er, Er0, er0, Er1, er1, Er2, er2, Er3, er3, Er4, er4, Er5, er5,  // ê
    Or, or, Or0, or0, Or1, or1, Or2, or2, Or3, or3, Or4, or4, Or5, or5,  // ô
    Oh, oh, Oh0, oh0, Oh1, oh1, Oh2, oh2, Oh3, oh3, Oh4, oh4, Oh5, oh5,  // ơ
    Uh, uh, Uh0, uh0, Uh1, uh1, Uh2, uh2, Uh3, uh3, Uh4, uh4, Uh5, uh5,  // ư
    
    LastChar,
}

impl VnLexiName {
    /// Check if this is a Vietnamese character
    pub fn is_vietnamese(&self) -> bool {
        !matches!(self, VnLexiName::NonVnChar)
    }
    
    /// Check if this is a vowel
    pub fn is_vowel(&self) -> bool {
        matches!(self, 
            VnLexiName::A | VnLexiName::a | VnLexiName::E | VnLexiName::e |
            VnLexiName::I | VnLexiName::i | VnLexiName::O | VnLexiName::o |
            VnLexiName::U | VnLexiName::u | VnLexiName::Y | VnLexiName::y |
            VnLexiName::Ar | VnLexiName::ar | VnLexiName::Ab | VnLexiName::ab |
            VnLexiName::Er | VnLexiName::er | VnLexiName::Or | VnLexiName::or |
            VnLexiName::Oh | VnLexiName::oh | VnLexiName::Uh | VnLexiName::uh |
            // All tone variants
            VnLexiName::A0 | VnLexiName::a0 | VnLexiName::A1 | VnLexiName::a1 |
            VnLexiName::A2 | VnLexiName::a2 | VnLexiName::A3 | VnLexiName::a3 |
            VnLexiName::A4 | VnLexiName::a4 | VnLexiName::A5 | VnLexiName::a5 |
            VnLexiName::E0 | VnLexiName::e0 | VnLexiName::E1 | VnLexiName::e1 |
            VnLexiName::E2 | VnLexiName::e2 | VnLexiName::E3 | VnLexiName::e3 |
            VnLexiName::E4 | VnLexiName::e4 | VnLexiName::E5 | VnLexiName::e5 |
            VnLexiName::I0 | VnLexiName::i0 | VnLexiName::I1 | VnLexiName::i1 |
            VnLexiName::I2 | VnLexiName::i2 | VnLexiName::I3 | VnLexiName::i3 |
            VnLexiName::I4 | VnLexiName::i4 | VnLexiName::I5 | VnLexiName::i5 |
            VnLexiName::O0 | VnLexiName::o0 | VnLexiName::O1 | VnLexiName::o1 |
            VnLexiName::O2 | VnLexiName::o2 | VnLexiName::O3 | VnLexiName::o3 |
            VnLexiName::O4 | VnLexiName::o4 | VnLexiName::O5 | VnLexiName::o5 |
            VnLexiName::U0 | VnLexiName::u0 | VnLexiName::U1 | VnLexiName::u1 |
            VnLexiName::U2 | VnLexiName::u2 | VnLexiName::U3 | VnLexiName::u3 |
            VnLexiName::U4 | VnLexiName::u4 | VnLexiName::U5 | VnLexiName::u5 |
            VnLexiName::Y0 | VnLexiName::y0 | VnLexiName::Y1 | VnLexiName::y1 |
            VnLexiName::Y2 | VnLexiName::y2 | VnLexiName::Y3 | VnLexiName::y3 |
            VnLexiName::Y4 | VnLexiName::y4 | VnLexiName::Y5 | VnLexiName::y5 |
            VnLexiName::Ar0 | VnLexiName::ar0 | VnLexiName::Ar1 | VnLexiName::ar1 |
            VnLexiName::Ar2 | VnLexiName::ar2 | VnLexiName::Ar3 | VnLexiName::ar3 |
            VnLexiName::Ar4 | VnLexiName::ar4 | VnLexiName::Ar5 | VnLexiName::ar5 |
            VnLexiName::Ab0 | VnLexiName::ab0 | VnLexiName::Ab1 | VnLexiName::ab1 |
            VnLexiName::Ab2 | VnLexiName::ab2 | VnLexiName::Ab3 | VnLexiName::ab3 |
            VnLexiName::Ab4 | VnLexiName::ab4 | VnLexiName::Ab5 | VnLexiName::ab5 |
            VnLexiName::Er0 | VnLexiName::er0 | VnLexiName::Er1 | VnLexiName::er1 |
            VnLexiName::Er2 | VnLexiName::er2 | VnLexiName::Er3 | VnLexiName::er3 |
            VnLexiName::Er4 | VnLexiName::er4 | VnLexiName::Er5 | VnLexiName::er5 |
            VnLexiName::Or0 | VnLexiName::or0 | VnLexiName::Or1 | VnLexiName::or1 |
            VnLexiName::Or2 | VnLexiName::or2 | VnLexiName::Or3 | VnLexiName::or3 |
            VnLexiName::Or4 | VnLexiName::or4 | VnLexiName::Or5 | VnLexiName::or5 |
            VnLexiName::Oh0 | VnLexiName::oh0 | VnLexiName::Oh1 | VnLexiName::oh1 |
            VnLexiName::Oh2 | VnLexiName::oh2 | VnLexiName::Oh3 | VnLexiName::oh3 |
            VnLexiName::Oh4 | VnLexiName::oh4 | VnLexiName::Oh5 | VnLexiName::oh5 |
            VnLexiName::Uh0 | VnLexiName::uh0 | VnLexiName::Uh1 | VnLexiName::uh1 |
            VnLexiName::Uh2 | VnLexiName::uh2 | VnLexiName::Uh3 | VnLexiName::uh3 |
            VnLexiName::Uh4 | VnLexiName::uh4 | VnLexiName::Uh5 | VnLexiName::uh5
        )
    }
    
    /// Get the base vowel (without tone marks)
    pub fn base_vowel(&self) -> Option<VnLexiName> {
        match self {
            VnLexiName::A | VnLexiName::a | VnLexiName::A0 | VnLexiName::a0 |
            VnLexiName::A1 | VnLexiName::a1 | VnLexiName::A2 | VnLexiName::a2 |
            VnLexiName::A3 | VnLexiName::a3 | VnLexiName::A4 | VnLexiName::a4 |
            VnLexiName::A5 | VnLexiName::a5 => Some(VnLexiName::A),
            
            VnLexiName::E | VnLexiName::e | VnLexiName::E0 | VnLexiName::e0 |
            VnLexiName::E1 | VnLexiName::e1 | VnLexiName::E2 | VnLexiName::e2 |
            VnLexiName::E3 | VnLexiName::e3 | VnLexiName::E4 | VnLexiName::e4 |
            VnLexiName::E5 | VnLexiName::e5 => Some(VnLexiName::E),
            
            VnLexiName::I | VnLexiName::i | VnLexiName::I0 | VnLexiName::i0 |
            VnLexiName::I1 | VnLexiName::i1 | VnLexiName::I2 | VnLexiName::i2 |
            VnLexiName::I3 | VnLexiName::i3 | VnLexiName::I4 | VnLexiName::i4 |
            VnLexiName::I5 | VnLexiName::i5 => Some(VnLexiName::I),
            
            VnLexiName::O | VnLexiName::o | VnLexiName::O0 | VnLexiName::o0 |
            VnLexiName::O1 | VnLexiName::o1 | VnLexiName::O2 | VnLexiName::o2 |
            VnLexiName::O3 | VnLexiName::o3 | VnLexiName::O4 | VnLexiName::o4 |
            VnLexiName::O5 | VnLexiName::o5 => Some(VnLexiName::O),
            
            VnLexiName::U | VnLexiName::u | VnLexiName::U0 | VnLexiName::u0 |
            VnLexiName::U1 | VnLexiName::u1 | VnLexiName::U2 | VnLexiName::u2 |
            VnLexiName::U3 | VnLexiName::u3 | VnLexiName::U4 | VnLexiName::u4 |
            VnLexiName::U5 | VnLexiName::u5 => Some(VnLexiName::U),
            
            VnLexiName::Y | VnLexiName::y | VnLexiName::Y0 | VnLexiName::y0 |
            VnLexiName::Y1 | VnLexiName::y1 | VnLexiName::Y2 | VnLexiName::y2 |
            VnLexiName::Y3 | VnLexiName::y3 | VnLexiName::Y4 | VnLexiName::y4 |
            VnLexiName::Y5 | VnLexiName::y5 => Some(VnLexiName::Y),
            
            VnLexiName::Ar | VnLexiName::ar | VnLexiName::Ar0 | VnLexiName::ar0 |
            VnLexiName::Ar1 | VnLexiName::ar1 | VnLexiName::Ar2 | VnLexiName::ar2 |
            VnLexiName::Ar3 | VnLexiName::ar3 | VnLexiName::Ar4 | VnLexiName::ar4 |
            VnLexiName::Ar5 | VnLexiName::ar5 => Some(VnLexiName::Ar),
            
            VnLexiName::Ab | VnLexiName::ab | VnLexiName::Ab0 | VnLexiName::ab0 |
            VnLexiName::Ab1 | VnLexiName::ab1 | VnLexiName::Ab2 | VnLexiName::ab2 |
            VnLexiName::Ab3 | VnLexiName::ab3 | VnLexiName::Ab4 | VnLexiName::ab4 |
            VnLexiName::Ab5 | VnLexiName::ab5 => Some(VnLexiName::Ab),
            
            VnLexiName::Er | VnLexiName::er | VnLexiName::Er0 | VnLexiName::er0 |
            VnLexiName::Er1 | VnLexiName::er1 | VnLexiName::Er2 | VnLexiName::er2 |
            VnLexiName::Er3 | VnLexiName::er3 | VnLexiName::Er4 | VnLexiName::er4 |
            VnLexiName::Er5 | VnLexiName::er5 => Some(VnLexiName::Er),
            
            VnLexiName::Or | VnLexiName::or | VnLexiName::Or0 | VnLexiName::or0 |
            VnLexiName::Or1 | VnLexiName::or1 | VnLexiName::Or2 | VnLexiName::or2 |
            VnLexiName::Or3 | VnLexiName::or3 | VnLexiName::Or4 | VnLexiName::or4 |
            VnLexiName::Or5 | VnLexiName::or5 => Some(VnLexiName::Or),
            
            VnLexiName::Oh | VnLexiName::oh | VnLexiName::Oh0 | VnLexiName::oh0 |
            VnLexiName::Oh1 | VnLexiName::oh1 | VnLexiName::Oh2 | VnLexiName::oh2 |
            VnLexiName::Oh3 | VnLexiName::oh3 | VnLexiName::Oh4 | VnLexiName::oh4 |
            VnLexiName::Oh5 | VnLexiName::oh5 => Some(VnLexiName::Oh),
            
            VnLexiName::Uh | VnLexiName::uh | VnLexiName::Uh0 | VnLexiName::uh0 |
            VnLexiName::Uh1 | VnLexiName::uh1 | VnLexiName::Uh2 | VnLexiName::uh2 |
            VnLexiName::Uh3 | VnLexiName::uh3 | VnLexiName::Uh4 | VnLexiName::uh4 |
            VnLexiName::Uh5 | VnLexiName::uh5 => Some(VnLexiName::Uh),
            
            _ => None,
        }
    }
    
    /// Get the tone number (0-5)
    pub fn tone(&self) -> u8 {
        match self {
            VnLexiName::A0 | VnLexiName::a0 | VnLexiName::E0 | VnLexiName::e0 |
            VnLexiName::I0 | VnLexiName::i0 | VnLexiName::O0 | VnLexiName::o0 |
            VnLexiName::U0 | VnLexiName::u0 | VnLexiName::Y0 | VnLexiName::y0 |
            VnLexiName::Ar0 | VnLexiName::ar0 | VnLexiName::Ab0 | VnLexiName::ab0 |
            VnLexiName::Er0 | VnLexiName::er0 | VnLexiName::Or0 | VnLexiName::or0 |
            VnLexiName::Oh0 | VnLexiName::oh0 | VnLexiName::Uh0 | VnLexiName::uh0 => 0,
            
            VnLexiName::A1 | VnLexiName::a1 | VnLexiName::E1 | VnLexiName::e1 |
            VnLexiName::I1 | VnLexiName::i1 | VnLexiName::O1 | VnLexiName::o1 |
            VnLexiName::U1 | VnLexiName::u1 | VnLexiName::Y1 | VnLexiName::y1 |
            VnLexiName::Ar1 | VnLexiName::ar1 | VnLexiName::Ab1 | VnLexiName::ab1 |
            VnLexiName::Er1 | VnLexiName::er1 | VnLexiName::Or1 | VnLexiName::or1 |
            VnLexiName::Oh1 | VnLexiName::oh1 | VnLexiName::Uh1 | VnLexiName::uh1 => 1,
            
            VnLexiName::A2 | VnLexiName::a2 | VnLexiName::E2 | VnLexiName::e2 |
            VnLexiName::I2 | VnLexiName::i2 | VnLexiName::O2 | VnLexiName::o2 |
            VnLexiName::U2 | VnLexiName::u2 | VnLexiName::Y2 | VnLexiName::y2 |
            VnLexiName::Ar2 | VnLexiName::ar2 | VnLexiName::Ab2 | VnLexiName::ab2 |
            VnLexiName::Er2 | VnLexiName::er2 | VnLexiName::Or2 | VnLexiName::or2 |
            VnLexiName::Oh2 | VnLexiName::oh2 | VnLexiName::Uh2 | VnLexiName::uh2 => 2,
            
            VnLexiName::A3 | VnLexiName::a3 | VnLexiName::E3 | VnLexiName::e3 |
            VnLexiName::I3 | VnLexiName::i3 | VnLexiName::O3 | VnLexiName::o3 |
            VnLexiName::U3 | VnLexiName::u3 | VnLexiName::Y3 | VnLexiName::y3 |
            VnLexiName::Ar3 | VnLexiName::ar3 | VnLexiName::Ab3 | VnLexiName::ab3 |
            VnLexiName::Er3 | VnLexiName::er3 | VnLexiName::Or3 | VnLexiName::or3 |
            VnLexiName::Oh3 | VnLexiName::oh3 | VnLexiName::Uh3 | VnLexiName::uh3 => 3,
            
            VnLexiName::A4 | VnLexiName::a4 | VnLexiName::E4 | VnLexiName::e4 |
            VnLexiName::I4 | VnLexiName::i4 | VnLexiName::O4 | VnLexiName::o4 |
            VnLexiName::U4 | VnLexiName::u4 | VnLexiName::Y4 | VnLexiName::y4 |
            VnLexiName::Ar4 | VnLexiName::ar4 | VnLexiName::Ab4 | VnLexiName::ab4 |
            VnLexiName::Er4 | VnLexiName::er4 | VnLexiName::Or4 | VnLexiName::or4 |
            VnLexiName::Oh4 | VnLexiName::oh4 | VnLexiName::Uh4 | VnLexiName::uh4 => 4,
            
            VnLexiName::A5 | VnLexiName::a5 | VnLexiName::E5 | VnLexiName::e5 |
            VnLexiName::I5 | VnLexiName::i5 | VnLexiName::O5 | VnLexiName::o5 |
            VnLexiName::U5 | VnLexiName::u5 | VnLexiName::Y5 | VnLexiName::y5 |
            VnLexiName::Ar5 | VnLexiName::ar5 | VnLexiName::Ab5 | VnLexiName::ab5 |
            VnLexiName::Er5 | VnLexiName::er5 | VnLexiName::Or5 | VnLexiName::or5 |
            VnLexiName::Oh5 | VnLexiName::oh5 | VnLexiName::Uh5 | VnLexiName::uh5 => 5,
            
            _ => 0,
        }
    }
    
    /// Check if this character has a tone mark
    pub fn has_tone(&self) -> bool {
        self.tone() > 0
    }
}

/// Vowel sequences - based on original UniKey implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum VowelSeq {
    Nil = -1,
    // Single vowels
    A, Ar, Ab, E, Er, I, O, Or, Oh, U, Uh, Y,
    // Two-vowel sequences
    Ai, Ao, Au, Ay, Aru, Ary, Eo, Eu, Eru, Ia, Ie, Ier, Iu,
    Oa, Oab, Oe, Oi, Ori, Ohi, Ua, Uar, Ue, Uer, Ui, Uo, Uor, Uoh, Uu, Uy,
    // Three-vowel sequences
    Uha, Uhi, Uho, Uhoh, Uhu, Ye, Yea, Yeu, Yia, Yie, Yiea, Yieo, Yieu, Yiu, 
    Yoa, Yoe, Yoi, You, Yua, Yue, Yui, Yuo, Yuy,
    LastSeq,
}

impl VowelSeq {
    /// Check if this is a valid vowel sequence
    pub fn is_valid(&self) -> bool {
        !matches!(self, VowelSeq::Nil)
    }
    
    /// Get the length of the vowel sequence
    pub fn length(&self) -> usize {
        match self {
            VowelSeq::Nil => 0,
            // Single vowels
            VowelSeq::A | VowelSeq::Ar | VowelSeq::Ab | VowelSeq::E | VowelSeq::Er |
            VowelSeq::I | VowelSeq::O | VowelSeq::Or | VowelSeq::Oh | VowelSeq::U |
            VowelSeq::Uh | VowelSeq::Y => 1,
            // Two-vowel sequences
            VowelSeq::Ai | VowelSeq::Ao | VowelSeq::Au | VowelSeq::Ay | VowelSeq::Aru |
            VowelSeq::Ary | VowelSeq::Eo | VowelSeq::Eu | VowelSeq::Eru | VowelSeq::Ia |
            VowelSeq::Ie | VowelSeq::Ier | VowelSeq::Iu | VowelSeq::Oa | VowelSeq::Oab |
            VowelSeq::Oe | VowelSeq::Oi | VowelSeq::Ori | VowelSeq::Ohi | VowelSeq::Ua |
            VowelSeq::Uar | VowelSeq::Ue | VowelSeq::Uer | VowelSeq::Ui | VowelSeq::Uo |
            VowelSeq::Uor | VowelSeq::Uoh | VowelSeq::Uu | VowelSeq::Uy => 2,
            // Three-vowel sequences
            VowelSeq::Uha | VowelSeq::Uhi | VowelSeq::Uho | VowelSeq::Uhoh | VowelSeq::Uhu |
            VowelSeq::Ye | VowelSeq::Yea | VowelSeq::Yeu | VowelSeq::Yia | VowelSeq::Yie |
            VowelSeq::Yiea | VowelSeq::Yieo | VowelSeq::Yieu | VowelSeq::Yiu | VowelSeq::Yoa |
            VowelSeq::Yoe | VowelSeq::Yoi | VowelSeq::You | VowelSeq::Yua | VowelSeq::Yue |
            VowelSeq::Yui | VowelSeq::Yuo | VowelSeq::Yuy => 3,
            _ => 0,
        }
    }
}

/// Key event types - based on original UniKey implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    // Diacritic marks
    RoofAll,    // ^ for all vowels
    RoofA,      // ^ for a
    RoofE,      // ^ for e  
    RoofO,      // ^ for o
    HookAll,    // + for all vowels
    HookUo,     // + for u, o
    HookU,      // + for u
    HookO,      // + for o
    Bowl,       // ( for u, o
    Dd,         // d for đ
    
    // Tone marks
    Tone0,      // No tone
    Tone1,      // Acute accent
    Tone2,      // Grave accent
    Tone3,      // Hook above
    Tone4,      // Tilde
    Tone5,      // Dot below
    
    // Special characters
    TelexW,     // Special w for telex
    MapChar,    // Mapped character
    EscChar,    // Escape character
    Normal,     // Normal character
}

/// Character types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Vn,         // Vietnamese character
    WordBreak,  // Word separator
    NonVn,      // Non-Vietnamese character
    Reset,      // Reset character
}

/// Key event - represents a key press event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub event_type: KeyEventType,
    pub char_type: CharType,
    pub vn_sym: VnLexiName,
    pub key_code: u32,
    pub tone: u8,
    pub modifiers: u32,  // Shift, Ctrl, Alt, etc.
}

impl KeyEvent {
    /// Create a new key event
    pub fn new(event_type: KeyEventType, char_type: CharType, vn_sym: VnLexiName, key_code: u32) -> Self {
        Self {
            event_type,
            char_type,
            vn_sym,
            key_code,
            tone: 0,
            modifiers: 0,
        }
    }
    
    /// Create a key event with tone
    pub fn with_tone(mut self, tone: u8) -> Self {
        self.tone = tone;
        self
    }
    
    /// Create a key event with modifiers
    pub fn with_modifiers(mut self, modifiers: u32) -> Self {
        self.modifiers = modifiers;
        self
    }
    
    /// Check if this is a Vietnamese character event
    pub fn is_vietnamese(&self) -> bool {
        matches!(self.char_type, CharType::Vn)
    }
    
    /// Check if this is a word break event
    pub fn is_word_break(&self) -> bool {
        matches!(self.char_type, CharType::WordBreak)
    }
    
    /// Check if this is a tone mark event
    pub fn is_tone_mark(&self) -> bool {
        matches!(self.event_type, 
            KeyEventType::Tone0 | KeyEventType::Tone1 | KeyEventType::Tone2 |
            KeyEventType::Tone3 | KeyEventType::Tone4 | KeyEventType::Tone5
        )
    }
    
    /// Check if this is a diacritic mark event
    pub fn is_diacritic(&self) -> bool {
        matches!(self.event_type,
            KeyEventType::RoofAll | KeyEventType::RoofA | KeyEventType::RoofE | KeyEventType::RoofO |
            KeyEventType::HookAll | KeyEventType::HookUo | KeyEventType::HookU | KeyEventType::HookO |
            KeyEventType::Bowl | KeyEventType::Dd
        )
    }
}

/// Input method types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    Telex,
    Vni,
    Viqr,
    User,
}

/// Output types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    Unicode,
    Utf8,
    Viqr,
    Tcvn3,
    Vni,
}
