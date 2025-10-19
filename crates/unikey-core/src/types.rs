//! Core data types for UniKey

/// Vietnamese lexical names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VnLexiName {
    NonVnChar = -1,
    A, a, A1, a1, A2, a2, A3, a3, A4, a4, A5, a5,
    Ar, ar, Ar1, ar1, Ar2, ar2, Ar3, ar3, Ar4, ar4, Ar5, ar5,
    Ab, ab, Ab1, ab1, Ab2, ab2, Ab3, ab3, Ab4, ab4, Ab5, ab5,
    B, b, C, c,
    D, d, DD, dd,
    E, e, E1, e1, E2, e2, E3, e3, E4, e4, E5, e5,
    Er, er, Er1, er1, Er2, er2, Er3, er3, Er4, er4, Er5, er5,
    F, f, G, g, H, h,
    I, i, I1, i1, I2, i2, I3, i3, I4, i4, I5, i5,
    J, j, K, k, L, l, M, m, N, n,
    O, o, O1, o1, O2, o2, O3, o3, O4, o4, O5, o5,
    Or, or, Or1, or1, Or2, or2, Or3, or3, Or4, or4, Or5, or5,
    Oh, oh, Oh1, oh1, Oh2, oh2, Oh3, oh3, Oh4, oh4, Oh5, oh5,
    P, p, Q, q, R, r, S, s, T, t,
    U, u, U1, u1, U2, u2, U3, u3, U4, u4, U5, u5,
    Uh, uh, Uh1, uh1, Uh2, uh2, Uh3, uh3, Uh4, uh4, Uh5, uh5,
    V, v, W, w, X, x,
    Y, y, Y1, y1, Y2, y2, Y3, y3, Y4, y4, Y5, y5,
    Z, z,
    LastChar,
}

/// Vowel sequences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VowelSeq {
    Nil = -1,
    A, Ar, Ab, E, Er, I, O, Or, Oh, U, Uh, Y,
    Ai, Ao, Au, Ay, Aru, Ary, Eo, Eu, Eru, Ia, Ie, Ier, Iu,
    Oa, Oab, Oe, Oi, Ori, Ohi, Ua, Uar, Ue, Uer, Ui, Uo, Uor, Uoh, Uu, Uy,
    Uha, Uhi, Uho, Uhoh, Uhu, Ye, Yea, Yeu, Yia, Yie, Yiea, Yieo, Yieu, Yiu, Yoa, Yoe, Yoi, You, Yua, Yue, Yui, Yuo, Yuy,
    LastSeq,
}

/// Key event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    RoofAll,
    RoofA,
    RoofE,
    RoofO,
    HookAll,
    HookUo,
    HookU,
    HookO,
    Bowl,
    Dd,
    Tone0,
    Tone1,
    Tone2,
    Tone3,
    Tone4,
    Tone5,
    TelexW,
    MapChar,
    EscChar,
    Normal,
}

/// Character types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Vn,
    WordBreak,
    NonVn,
    Reset,
}

/// Key event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub event_type: KeyEventType,
    pub char_type: CharType,
    pub vn_sym: VnLexiName,
    pub key_code: u32,
    pub tone: i32,
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
