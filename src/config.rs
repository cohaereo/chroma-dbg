#[derive(Debug, Clone)]
pub struct ChromaConfig {
    pub inline_struct: InlineThreshold,
    pub inline_array: InlineThreshold,
    pub integer_format: IntegerFormat,

    pub identifier_color: Color,
    /// Color used for integers, floats, and booleans.
    pub numerical_color: Color,
    /// Color used for string literals.
    pub string_color: Color,
    /// Color used for field names.
    pub field_color: Color,
}

impl ChromaConfig {
    pub const DEFAULT: ChromaConfig = ChromaConfig {
        inline_struct: InlineThreshold::MaxLength(64),
        inline_array: InlineThreshold::MaxLength(64),
        integer_format: IntegerFormat::HexWhenOver(8192),

        identifier_color: Color(19, 220, 242),
        numerical_color: Color(200, 129, 255),
        string_color: Color(232, 219, 97),
        field_color: Color(255, 255, 255),
    };

    pub const COMPACT: ChromaConfig = ChromaConfig {
        inline_struct: InlineThreshold::Always,
        inline_array: InlineThreshold::Always,
        ..Self::DEFAULT
    };
}

impl Default for ChromaConfig {
    fn default() -> Self {
        ChromaConfig::DEFAULT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineThreshold {
    /// Never inline
    Never,
    /// Always inline
    Always,
    /// Maximum length of a single line before it is considered too long to be inline.
    MaxLength(usize),
}

impl InlineThreshold {
    pub fn should_inline(&self, len: usize) -> bool {
        match self {
            InlineThreshold::Never => false,
            InlineThreshold::Always => true,
            InlineThreshold::MaxLength(max) => len <= *max,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerFormat {
    /// Always render integers as decimal.
    AlwaysDecimal,
    /// Always render integers as hexadecimal. Note that negative numbers will still be rendered as decimal.
    AlwaysHex,
    /// Render integers as hexadecimal when they are over a certain size.
    HexWhenOver(u64),
}

impl IntegerFormat {
    pub fn format(&self, value: u64) -> String {
        match self {
            IntegerFormat::AlwaysDecimal => value.to_string(),
            IntegerFormat::AlwaysHex => format!("0x{:x}", value),
            IntegerFormat::HexWhenOver(max) if value > *max => format!("0x{:x}", value),
            IntegerFormat::HexWhenOver(_) => value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Color> for anstyle::RgbColor {
    fn from(color: Color) -> Self {
        Self(color.0, color.1, color.2)
    }
}
