use crate::CodePoint;

mod ids;
pub use ids::*;

pub const NULL: CodePoint = 0x00;
pub const BACKSPACE: CodePoint = 0x08;
pub const CHARACTER_TABULATION: CodePoint = 0x09;
pub const LINE_FEED: CodePoint = 0x0a;
pub const LINE_TABULATION: CodePoint = 0x0b;
pub const FORM_FEED: CodePoint = 0x0c;
pub const CARRIAGE_RETURN: CodePoint = 0x0d;
pub const EXCLAMATION_MARK: CodePoint = 0x21;
pub const NUMBER_SIGN: CodePoint = 0x23;
pub const DOLLAR_SIGN: CodePoint = 0x24;
pub const PERCENT_SIGN: CodePoint = 0x25;
pub const AMPERSAND: CodePoint = 0x26;
pub const LEFT_PARENTHESIS: CodePoint = 0x28;
pub const RIGHT_PARENTHESIS: CodePoint = 0x29;
pub const ASTERISK: CodePoint = 0x2a;
pub const PLUS_SIGN: CodePoint = 0x2b;
pub const COMMA: CodePoint = 0x2c;
pub const HYPHEN_MINUS: CodePoint = 0x2d;
pub const FULL_STOP: CodePoint = 0x2e;
pub const SOLIDUS: CodePoint = 0x2f;
pub const DIGIT_ZERO: CodePoint = 0x30;
pub const DIGIT_ONE: CodePoint = 0x31;
pub const DIGIT_SEVEN: CodePoint = 0x37;
pub const DIGIT_NINE: CodePoint = 0x39;
pub const COLON: CodePoint = 0x3a;
pub const SEMICOLON: CodePoint = 0x3b;
pub const LESS_THAN_SIGN: CodePoint = 0x3c;
pub const EQUALS_SIGN: CodePoint = 0x3d;
pub const GREATER_THAN_SIGN: CodePoint = 0x3e;
pub const QUESTION_MARK: CodePoint = 0x3f;
pub const COMMERCIAL_AT: CodePoint = 0x40;
pub const LATIN_CAPITAL_LETTER_A: CodePoint = 0x41;
pub const LATIN_CAPITAL_LETTER_B: CodePoint = 0x42;
pub const LATIN_CAPITAL_LETTER_D: CodePoint = 0x44;
pub const LATIN_CAPITAL_LETTER_F: CodePoint = 0x46;
pub const LATIN_CAPITAL_LETTER_P: CodePoint = 0x50;
pub const LATIN_CAPITAL_LETTER_S: CodePoint = 0x53;
pub const LATIN_CAPITAL_LETTER_W: CodePoint = 0x57;
pub const LATIN_CAPITAL_LETTER_Z: CodePoint = 0x5a;
pub const LOW_LINE: CodePoint = 0x5f;
pub const LATIN_SMALL_LETTER_A: CodePoint = 0x61;
pub const LATIN_SMALL_LETTER_B: CodePoint = 0x62;
pub const LATIN_SMALL_LETTER_C: CodePoint = 0x63;
pub const LATIN_SMALL_LETTER_D: CodePoint = 0x64;
pub const LATIN_SMALL_LETTER_F: CodePoint = 0x66;
pub const LATIN_SMALL_LETTER_G: CodePoint = 0x67;
pub const LATIN_SMALL_LETTER_I: CodePoint = 0x69;
pub const LATIN_SMALL_LETTER_K: CodePoint = 0x6b;
pub const LATIN_SMALL_LETTER_M: CodePoint = 0x6d;
pub const LATIN_SMALL_LETTER_N: CodePoint = 0x6e;
pub const LATIN_SMALL_LETTER_P: CodePoint = 0x70;
pub const LATIN_SMALL_LETTER_Q: CodePoint = 0x71;
pub const LATIN_SMALL_LETTER_R: CodePoint = 0x72;
pub const LATIN_SMALL_LETTER_S: CodePoint = 0x73;
pub const LATIN_SMALL_LETTER_T: CodePoint = 0x74;
pub const LATIN_SMALL_LETTER_U: CodePoint = 0x75;
pub const LATIN_SMALL_LETTER_V: CodePoint = 0x76;
pub const LATIN_SMALL_LETTER_W: CodePoint = 0x77;
pub const LATIN_SMALL_LETTER_X: CodePoint = 0x78;
pub const LATIN_SMALL_LETTER_Y: CodePoint = 0x79;
pub const LATIN_SMALL_LETTER_Z: CodePoint = 0x7a;
pub const LEFT_SQUARE_BRACKET: CodePoint = 0x5b;
pub const REVERSE_SOLIDUS: CodePoint = 0x5c;
pub const RIGHT_SQUARE_BRACKET: CodePoint = 0x5d;
pub const CIRCUMFLEX_ACCENT: CodePoint = 0x5e;
pub const GRAVE_ACCENT: CodePoint = 0x60;
pub const LEFT_CURLY_BRACKET: CodePoint = 0x7b;
pub const VERTICAL_LINE: CodePoint = 0x7c;
pub const RIGHT_CURLY_BRACKET: CodePoint = 0x7d;
pub const TILDE: CodePoint = 0x7e;
pub const ZERO_WIDTH_NON_JOINER: CodePoint = 0x200c;
pub const ZERO_WIDTH_JOINER: CodePoint = 0x200d;
pub const LINE_SEPARATOR: CodePoint = 0x2028;
pub const PARAGRAPH_SEPARATOR: CodePoint = 0x2029;

pub fn is_decimal_digit(code: CodePoint) -> bool {
    code >= DIGIT_ZERO && code <= DIGIT_NINE
}

pub fn is_line_terminator(code: CodePoint) -> bool {
    matches!(
        code,
        LINE_FEED | CARRIAGE_RETURN | LINE_SEPARATOR | PARAGRAPH_SEPARATOR
    )
}

pub fn digit_to_int(code: CodePoint) -> CodePoint {
    if code >= LATIN_SMALL_LETTER_A && code <= LATIN_SMALL_LETTER_F {
        return code - LATIN_SMALL_LETTER_A + 10;
    }
    if code >= LATIN_CAPITAL_LETTER_A && code <= LATIN_CAPITAL_LETTER_F {
        return code - LATIN_CAPITAL_LETTER_A + 10;
    }
    code - DIGIT_ZERO
}

pub fn is_lead_surrogate(code: CodePoint) -> bool {
    code >= 0xd800 && code <= 0xdbff
}

pub fn is_trail_surrogate(code: CodePoint) -> bool {
    code >= 0xdc00 && code <= 0xdfff
}

pub fn combine_surrogate_pair(lead: CodePoint, trail: CodePoint) -> CodePoint {
    (lead - 0xd800) * 0x400 + (trail - 0xdc00) + 0x10000
}
