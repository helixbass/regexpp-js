use std::{collections::HashSet, rc::Rc};

use derive_builder::Builder;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use squalid::OptionExt;

use crate::{
    ecma_versions::LATEST_ECMA_VERSION,
    reader::CodePoint,
    regexp_syntax_error::{self, new_reg_exp_syntax_error},
    unicode::{
        is_line_terminator, AMPERSAND, ASTERISK, BACKSPACE, CIRCUMFLEX_ACCENT, COLON, COMMA,
        COMMERCIAL_AT, DOLLAR_SIGN, EQUALS_SIGN, EXCLAMATION_MARK, FULL_STOP, GRAVE_ACCENT,
        GREATER_THAN_SIGN, HYPHEN_MINUS, LATIN_CAPITAL_LETTER_B, LATIN_SMALL_LETTER_B,
        LATIN_SMALL_LETTER_C, LATIN_SMALL_LETTER_D, LATIN_SMALL_LETTER_G, LATIN_SMALL_LETTER_I,
        LATIN_SMALL_LETTER_M, LATIN_SMALL_LETTER_Q, LATIN_SMALL_LETTER_S, LATIN_SMALL_LETTER_U,
        LATIN_SMALL_LETTER_V, LATIN_SMALL_LETTER_Y, LEFT_CURLY_BRACKET, LEFT_PARENTHESIS,
        LEFT_SQUARE_BRACKET, LESS_THAN_SIGN, NUMBER_SIGN, PERCENT_SIGN, PLUS_SIGN, QUESTION_MARK,
        REVERSE_SOLIDUS, RIGHT_CURLY_BRACKET, RIGHT_PARENTHESIS, RIGHT_SQUARE_BRACKET, SEMICOLON,
        SOLIDUS, TILDE, VERTICAL_LINE,
    },
    EcmaVersion, Reader, RegExpSyntaxError,
};

static SYNTAX_CHARACTER: Lazy<HashSet<CodePoint>> = Lazy::new(|| {
    [
        CIRCUMFLEX_ACCENT,
        DOLLAR_SIGN,
        REVERSE_SOLIDUS,
        FULL_STOP,
        ASTERISK,
        PLUS_SIGN,
        QUESTION_MARK,
        LEFT_PARENTHESIS,
        RIGHT_PARENTHESIS,
        LEFT_SQUARE_BRACKET,
        RIGHT_SQUARE_BRACKET,
        LEFT_CURLY_BRACKET,
        RIGHT_CURLY_BRACKET,
        VERTICAL_LINE,
    ]
    .into_iter()
    .collect()
});

static CLASS_SET_RESERVED_DOUBLE_PUNCTUATOR_CHARACTER: Lazy<HashSet<CodePoint>> = Lazy::new(|| {
    [
        AMPERSAND,
        EXCLAMATION_MARK,
        NUMBER_SIGN,
        DOLLAR_SIGN,
        PERCENT_SIGN,
        ASTERISK,
        PLUS_SIGN,
        COMMA,
        FULL_STOP,
        COLON,
        SEMICOLON,
        LESS_THAN_SIGN,
        EQUALS_SIGN,
        GREATER_THAN_SIGN,
        QUESTION_MARK,
        COMMERCIAL_AT,
        CIRCUMFLEX_ACCENT,
        GRAVE_ACCENT,
        TILDE,
    ]
    .into_iter()
    .collect()
});

static CLASS_SET_SYNTAX_CHARACTER: Lazy<HashSet<CodePoint>> = Lazy::new(|| {
    [
        LEFT_PARENTHESIS,
        RIGHT_PARENTHESIS,
        LEFT_SQUARE_BRACKET,
        RIGHT_SQUARE_BRACKET,
        LEFT_CURLY_BRACKET,
        RIGHT_CURLY_BRACKET,
        SOLIDUS,
        HYPHEN_MINUS,
        REVERSE_SOLIDUS,
        VERTICAL_LINE,
    ]
    .into_iter()
    .collect()
});

static CLASS_SET_RESERVED_PUNCTUATOR: Lazy<HashSet<CodePoint>> = Lazy::new(|| {
    [
        AMPERSAND,
        HYPHEN_MINUS,
        EXCLAMATION_MARK,
        NUMBER_SIGN,
        PERCENT_SIGN,
        COMMA,
        COLON,
        SEMICOLON,
        LESS_THAN_SIGN,
        EQUALS_SIGN,
        GREATER_THAN_SIGN,
        COMMERCIAL_AT,
        GRAVE_ACCENT,
        TILDE,
    ]
    .into_iter()
    .collect()
});

fn is_syntax_character(cp: CodePoint) -> bool {
    SYNTAX_CHARACTER.contains(&cp)
}

fn is_class_set_reserved_double_punctuator_character(cp: Option<CodePoint>) -> bool {
    cp.matches(|cp| CLASS_SET_RESERVED_DOUBLE_PUNCTUATOR_CHARACTER.contains(&cp))
}

fn is_class_set_syntax_character(cp: CodePoint) -> bool {
    CLASS_SET_SYNTAX_CHARACTER.contains(&cp)
}

fn is_class_set_reserved_punctuator(cp: Option<CodePoint>) -> bool {
    cp.matches(|cp| CLASS_SET_RESERVED_PUNCTUATOR.contains(&cp))
}

#[derive(Copy, Clone, Default)]
struct UnicodeSetsConsumeResult {
    may_contain_strings: Option<bool>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegExpValidatorSourceContextKind {
    Flags,
    Literal,
    Pattern,
}

pub struct RegExpValidatorSourceContext {
    pub source: Vec<u16>,
    pub start: usize,
    pub end: usize,
    pub kind: RegExpValidatorSourceContextKind,
}

pub struct RegExpFlags {
    pub global: bool,
    pub ignore_case: bool,
    pub multiline: bool,
    pub unicode: bool,
    pub sticky: bool,
    pub dot_all: bool,
    pub has_indices: bool,
    pub unicode_sets: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssertionKind {
    Lookahead,
    Lookbehind,
    End,
    Start,
    Word,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CharacterKind {
    Any,
    Digit,
    Space,
    Word,
    Property,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum CapturingGroupKey {
    Index(usize),
    Name(Vec<u16>),
}

pub trait Options {
    fn strict(&self) -> Option<bool>;
    fn ecma_version(&self) -> Option<EcmaVersion>;
    fn on_literal_enter(&self, start: usize) {}
    fn on_literal_leave(&self, start: usize, end: usize) {}
    fn on_reg_exp_flags(&self, start: usize, end: usize, flags: RegExpFlags) {}
    fn on_pattern_enter(&self, start: usize) {}
    fn on_pattern_leave(&self, start: usize, end: usize) {}
    fn on_disjunction_enter(&self, start: usize) {}
    fn on_disjunction_leave(&self, start: usize, end: usize) {}
    fn on_alternative_enter(&self, start: usize, index: usize) {}
    fn on_alternative_leave(&self, start: usize, end: usize, index: usize) {}
    fn on_group_enter(&self, start: usize) {}
    fn on_group_leave(&self, start: usize, end: usize) {}
    fn on_capturing_group_enter(&self, start: usize, name: Option<&[u16]>) {}
    fn on_capturing_group_leave(&self, start: usize, end: usize, name: Option<&[u16]>) {}
    fn on_quantifier(&self, start: usize, end: usize, min: usize, max: usize, greedy: bool) {}
    fn on_lookaround_assertion_enter(&self, start: usize, kind: AssertionKind, negate: bool) {}
    fn on_lookaround_assertion_leave(
        &self,
        start: usize,
        end: usize,
        kind: AssertionKind,
        negate: bool,
    ) {
    }
    fn on_edge_assertion(&self, start: usize, end: usize, kind: AssertionKind) {}
    fn on_word_boundary_assertion(
        &self,
        start: usize,
        end: usize,
        kind: AssertionKind,
        negate: bool,
    ) {
    }
    fn on_any_character_set(&self, start: usize, end: usize, kind: CharacterKind) {}
    fn on_escape_character_set(&self, start: usize, end: usize, kind: CharacterKind, negate: bool) {
    }
    fn on_unicode_property_character_set(
        &self,
        start: usize,
        end: usize,
        kind: CharacterKind,
        key: &[u16],
        value: Option<&[u16]>,
        negate: bool,
        strings: bool,
    ) {
    }
    fn on_character(&self, start: usize, end: usize, value: CodePoint) {}
    fn on_backreference(&self, start: usize, end: usize, ref_: &CapturingGroupKey) {}
    fn on_character_class_enter(&self, start: usize, negate: bool, unicode_sets: bool) {}
    fn on_character_class_leave(&self, start: usize, end: usize, negate: bool) {}
    fn on_character_class_range(&self, start: usize, end: usize, min: CodePoint, max: CodePoint) {}
    fn on_class_intersection(&self, start: usize, end: usize) {}
    fn on_class_subtraction(&self, start: usize, end: usize) {}
    fn on_class_string_disjunction_enter(&self, start: usize) {}
    fn on_class_string_disjunction_leave(&self, start: usize, end: usize) {}
    fn on_string_alternative_enter(&self, start: usize, index: usize) {}
    fn on_string_alternative_leave(&self, start: usize, end: usize, index: usize) {}
}

struct NoopOptions;

impl Options for NoopOptions {
    fn strict(&self) -> Option<bool> {
        None
    }

    fn ecma_version(&self) -> Option<EcmaVersion> {
        None
    }
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ValidatePatternFlags {
    unicode: Option<bool>,
    unicode_sets: Option<bool>,
}

struct Mode {
    unicode_mode: bool,
    n_flag: bool,
    unicode_sets_mode: bool,
}

#[derive(Builder, Copy, Clone, Default)]
#[builder(default, setter(strip_option))]
struct RaiseContext {
    index: Option<usize>,
    unicode: Option<bool>,
    unicode_sets: Option<bool>,
}

fn bmp_char_to_utf_16(ch: char) -> u16 {
    let mut buffer = [0; 1];
    ch.encode_utf16(&mut buffer);
    buffer[0]
}

pub struct RegExpValidator<'a> {
    _options: Rc<dyn Options + 'a>,
    _reader: Reader,
    _unicode_mode: bool,
    _unicode_sets_mode: bool,
    _n_flag: bool,
    _last_int_value: Option<CodePoint>,
    _last_assertion_is_quantifiable: bool,
    _num_capturing_parens: usize,
    _group_names: HashSet<String>,
    _backreference_names: HashSet<String>,
    _src_ctx: Option<RegExpValidatorSourceContext>,
}

impl<'a> RegExpValidator<'a> {
    pub fn new(options: Option<Rc<dyn Options + 'a>>) -> Self {
        Self {
            _options: options.unwrap_or_else(|| Rc::new(NoopOptions)),
            _reader: Default::default(),
            _unicode_mode: Default::default(),
            _unicode_sets_mode: Default::default(),
            _n_flag: Default::default(),
            _last_int_value: Some(Default::default()),
            _last_assertion_is_quantifiable: Default::default(),
            _num_capturing_parens: Default::default(),
            _group_names: Default::default(),
            _backreference_names: Default::default(),
            _src_ctx: Default::default(),
        }
    }

    pub fn validate_literal(
        &mut self,
        source: &[u16],
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(), RegExpSyntaxError> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(source.len());
        self._src_ctx = Some(RegExpValidatorSourceContext {
            source: source.to_owned(),
            start,
            end,
            kind: RegExpValidatorSourceContextKind::Literal,
        });
        self._unicode_sets_mode = false;
        self._unicode_mode = false;
        self._n_flag = false;
        self.reset(source, start, end);

        self.on_literal_enter(start);
        if self.eat(SOLIDUS) && self.eat_reg_exp_body()? && self.eat(SOLIDUS) {
            let flag_start = self.index();
            let unicode = source[flag_start..].contains(&bmp_char_to_utf_16('u'));
            let unicode_sets = source[flag_start..].contains(&bmp_char_to_utf_16('v'));
            self.validate_flags_internal(source, flag_start, end)?;
            self.validate_pattern_internal(
                source,
                start + 1,
                flag_start - 1,
                Some(ValidatePatternFlags {
                    unicode: Some(unicode),
                    unicode_sets: Some(unicode_sets),
                }),
            )?;
        } else if start >= end {
            self.raise("Empty", None)?;
        } else {
            let c = char::try_from(self.current_code_point().unwrap()).unwrap();
            self.raise(&format!("Unexpected character '{c}'"), None)?;
        }
        self.on_literal_leave(start, end);
        Ok(())
    }

    pub fn validate_flags(
        &mut self,
        source: &[u16],
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<(), RegExpSyntaxError> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(source.len());
        self._src_ctx = Some(RegExpValidatorSourceContext {
            source: source.to_owned(),
            start,
            end,
            kind: RegExpValidatorSourceContextKind::Flags,
        });
        self.validate_flags_internal(source, start, end)
    }

    pub fn validate_pattern(
        &mut self,
        source: &[u16],
        start: Option<usize>,
        end: Option<usize>,
        flags: Option<ValidatePatternFlags>,
    ) -> Result<(), RegExpSyntaxError> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(source.len());
        self._src_ctx = Some(RegExpValidatorSourceContext {
            source: source.to_owned(),
            start,
            end,
            kind: RegExpValidatorSourceContextKind::Pattern,
        });
        self.validate_pattern_internal(source, start, end, flags)
    }

    fn validate_pattern_internal(
        &mut self,
        source: &[u16],
        start: usize,
        end: usize,
        flags: Option<ValidatePatternFlags>,
    ) -> Result<(), RegExpSyntaxError> {
        let mode = self._parse_flags_option_to_mode(flags, end)?;

        self._unicode_mode = mode.unicode_mode;
        self._n_flag = mode.n_flag;
        self._unicode_sets_mode = mode.unicode_sets_mode;
        self.reset(source, start, end);
        self.consume_pattern()?;

        if !self._n_flag
            && self.ecma_version() >= EcmaVersion::_2018
            && !self._group_names.is_empty()
        {
            self._n_flag = true;
            self.rewind(start);
        }

        Ok(())
    }

    fn validate_flags_internal(
        &mut self,
        source: &[u16],
        start: usize,
        end: usize,
    ) -> Result<(), RegExpSyntaxError> {
        let mut existing_flags: HashSet<CodePoint> = Default::default();
        let mut global = false;
        let mut ignore_case = false;
        let mut multiline = false;
        let mut sticky = false;
        let mut unicode = false;
        let mut dot_all = false;
        let mut has_indices = false;
        let mut unicode_sets = false;
        for &flag in source {
            let flag: CodePoint = flag.into();

            if existing_flags.contains(&flag) {
                // TODO: technically probably should handle the failure of char::try_from()
                // here (and below, and in validate_literal()) which I believe would fail
                // if this is part of a surrogate pair?
                self.raise(
                    &format!("Duplicated flag '{}'", char::try_from(flag).unwrap()),
                    Some(RaiseContextBuilder::default().index(start).build().unwrap()),
                )?;
            }
            existing_flags.insert(flag);

            if flag == LATIN_SMALL_LETTER_G {
                global = true;
            } else if flag == LATIN_SMALL_LETTER_I {
                ignore_case = true;
            } else if flag == LATIN_SMALL_LETTER_M {
                multiline = true;
            } else if flag == LATIN_SMALL_LETTER_U && self.ecma_version() >= EcmaVersion::_2015 {
                unicode = true;
            } else if flag == LATIN_SMALL_LETTER_Y && self.ecma_version() >= EcmaVersion::_2015 {
                sticky = true;
            } else if flag == LATIN_SMALL_LETTER_S && self.ecma_version() >= EcmaVersion::_2018 {
                dot_all = true;
            } else if flag == LATIN_SMALL_LETTER_D && self.ecma_version() >= EcmaVersion::_2022 {
                has_indices = true;
            } else if flag == LATIN_SMALL_LETTER_V && self.ecma_version() >= EcmaVersion::_2024 {
                unicode_sets = true;
            } else {
                self.raise(
                    &format!("Invalid flag '{}'", char::try_from(flag).unwrap()),
                    Some(RaiseContextBuilder::default().index(start).build().unwrap()),
                )?;
            }
        }
        self.on_reg_exp_flags(
            start,
            end,
            RegExpFlags {
                global,
                ignore_case,
                multiline,
                unicode,
                sticky,
                dot_all,
                has_indices,
                unicode_sets,
            },
        );
        Ok(())
    }

    fn strict(&self) -> bool {
        self._options.strict().unwrap_or_default() || self._unicode_mode
    }

    fn ecma_version(&self) -> EcmaVersion {
        self._options.ecma_version().unwrap_or(LATEST_ECMA_VERSION)
    }

    fn on_literal_enter(&mut self, start: usize) {
        self._options.on_literal_enter(start);
    }

    fn on_literal_leave(&mut self, start: usize, end: usize) {
        self._options.on_literal_leave(start, end);
    }

    fn on_reg_exp_flags(&mut self, start: usize, end: usize, flags: RegExpFlags) {
        self._options.on_reg_exp_flags(start, end, flags);
    }

    fn on_pattern_enter(&mut self, start: usize) {
        self._options.on_pattern_enter(start);
    }

    fn on_pattern_leave(&mut self, start: usize, end: usize) {
        self._options.on_pattern_leave(start, end);
    }

    fn on_disjunction_enter(&mut self, start: usize) {
        self._options.on_disjunction_enter(start);
    }

    fn on_disjunction_leave(&mut self, start: usize, end: usize) {
        self._options.on_disjunction_leave(start, end);
    }

    fn on_alternative_enter(&mut self, start: usize, index: usize) {
        self._options.on_alternative_enter(start, index);
    }

    fn on_alternative_leave(&mut self, start: usize, end: usize, index: usize) {
        self._options.on_alternative_leave(start, end, index);
    }

    fn on_lookaround_assertion_enter(&mut self, start: usize, kind: AssertionKind, negate: bool) {
        self._options
            .on_lookaround_assertion_enter(start, kind, negate);
    }

    fn on_lookaround_assertion_leave(
        &mut self,
        start: usize,
        end: usize,
        kind: AssertionKind,
        negate: bool,
    ) {
        self._options
            .on_lookaround_assertion_leave(start, end, kind, negate);
    }

    fn on_edge_assertion(&mut self, start: usize, end: usize, kind: AssertionKind) {
        self._options.on_edge_assertion(start, end, kind);
    }

    fn on_word_boundary_assertion(
        &mut self,
        start: usize,
        end: usize,
        kind: AssertionKind,
        negate: bool,
    ) {
        self._options
            .on_word_boundary_assertion(start, end, kind, negate);
    }

    fn on_any_character_set(&mut self, start: usize, end: usize, kind: CharacterKind) {
        self._options.on_any_character_set(start, end, kind);
    }

    fn on_character(&mut self, start: usize, end: usize, value: CodePoint) {
        self._options.on_character(start, end, value);
    }

    fn on_character_class_enter(&mut self, start: usize, negate: bool, unicode_sets: bool) {
        self._options
            .on_character_class_enter(start, negate, unicode_sets);
    }

    fn on_character_class_leave(&mut self, start: usize, end: usize, negate: bool) {
        self._options.on_character_class_leave(start, end, negate);
    }

    fn on_character_class_range(
        &mut self,
        start: usize,
        end: usize,
        min: CodePoint,
        max: CodePoint,
    ) {
        self._options.on_character_class_range(start, end, min, max);
    }

    fn on_class_intersection(&mut self, start: usize, end: usize) {
        self._options.on_class_intersection(start, end);
    }

    fn on_class_subtraction(&mut self, start: usize, end: usize) {
        self._options.on_class_subtraction(start, end);
    }

    fn on_class_string_disjunction_enter(&mut self, start: usize) {
        self._options.on_class_string_disjunction_enter(start);
    }

    fn on_class_string_disjunction_leave(&mut self, start: usize, end: usize) {
        self._options.on_class_string_disjunction_leave(start, end);
    }

    fn _parse_flags_option_to_mode(
        &self,
        flags: Option<ValidatePatternFlags>,
        source_end: usize,
    ) -> Result<Mode, RegExpSyntaxError> {
        let mut unicode = false;
        let mut unicode_sets = false;
        if let Some(flags) = flags.filter(|_| self.ecma_version() >= EcmaVersion::_2015) {
            unicode = flags.unicode == Some(true);
            if self.ecma_version() >= EcmaVersion::_2024 {
                unicode_sets = flags.unicode_sets == Some(true);
            }
        }

        if unicode && unicode_sets {
            self.raise(
                "Invalid regular expression flags",
                Some(RaiseContext {
                    index: Some(source_end + 1),
                    unicode: Some(unicode),
                    unicode_sets: Some(unicode_sets),
                }),
            )?;
        }

        let unicode_mode = unicode || unicode_sets;
        let n_flag = unicode && self.ecma_version() >= EcmaVersion::_2018
            || unicode_sets
            || self._options.strict() == Some(true) && self.ecma_version() >= EcmaVersion::_2023;
        let unicode_sets_mode = unicode_sets;

        Ok(Mode {
            unicode_mode,
            n_flag,
            unicode_sets_mode,
        })
    }

    fn index(&self) -> usize {
        self._reader.index()
    }

    fn current_code_point(&self) -> Option<CodePoint> {
        self._reader.current_code_point()
    }

    fn next_code_point(&self) -> Option<CodePoint> {
        self._reader.next_code_point()
    }

    fn next_code_point2(&self) -> Option<CodePoint> {
        self._reader.next_code_point2()
    }

    fn next_code_point3(&self) -> Option<CodePoint> {
        self._reader.next_code_point3()
    }

    fn reset(&mut self, source: &[u16], start: usize, end: usize) {
        self._reader.reset(source, start, end, self._unicode_mode);
    }

    fn rewind(&mut self, index: usize) {
        self._reader.rewind(index);
    }

    fn advance(&mut self) {
        self._reader.advance();
    }

    fn eat(&mut self, cp: CodePoint) -> bool {
        self._reader.eat(cp)
    }

    fn eat2(&mut self, cp1: CodePoint, cp2: CodePoint) -> bool {
        self._reader.eat2(cp1, cp2)
    }

    fn eat3(&mut self, cp1: CodePoint, cp2: CodePoint, cp3: CodePoint) -> bool {
        self._reader.eat3(cp1, cp2, cp3)
    }

    fn raise(&self, message: &str, context: Option<RaiseContext>) -> Result<(), RegExpSyntaxError> {
        Err(new_reg_exp_syntax_error(
            self._src_ctx.as_ref().unwrap(),
            regexp_syntax_error::Flags {
                unicode: context
                    .and_then(|context| context.unicode)
                    .unwrap_or(self._unicode_mode && !self._unicode_sets_mode),
                unicode_sets: context
                    .and_then(|context| context.unicode_sets)
                    .unwrap_or(self._unicode_sets_mode),
            },
            context
                .and_then(|context| context.index)
                .unwrap_or(self.index()),
            message,
        ))
    }

    fn eat_reg_exp_body(&mut self) -> Result<bool, RegExpSyntaxError> {
        let start = self.index();
        let mut in_class = false;
        let mut escaped = false;

        loop {
            let cp = self.current_code_point();
            if match cp {
                None => true,
                Some(cp) => is_line_terminator(cp),
            } {
                let kind = if in_class {
                    "character class"
                } else {
                    "regular expression"
                };
                self.raise(&format!("Unterminated {kind}"), None)?;
            }
            let cp = cp.unwrap();
            if escaped {
                escaped = false;
            } else if cp == REVERSE_SOLIDUS {
                escaped = true;
            } else if cp == LEFT_SQUARE_BRACKET {
                in_class = true;
            } else if cp == RIGHT_SQUARE_BRACKET {
                in_class = false;
            } else if cp == SOLIDUS && !in_class || cp == ASTERISK && self.index() == start {
                break;
            }
            self.advance();
        }

        Ok(self.index() != start)
    }

    fn consume_pattern(&mut self) -> Result<(), RegExpSyntaxError> {
        let start = self.index();
        self._num_capturing_parens = self.count_capturing_parens();
        self._group_names.clear();
        self._backreference_names.clear();

        self.on_pattern_enter(start);
        self.consume_disjunction()?;

        let cp = self.current_code_point();
        if let Some(cp) = cp {
            if cp == RIGHT_PARENTHESIS {
                self.raise("Unmatched ')'", None)?;
            }
            if cp == REVERSE_SOLIDUS {
                self.raise("\\ at end of pattern", None)?;
            }
            if cp == RIGHT_SQUARE_BRACKET || cp == RIGHT_CURLY_BRACKET {
                self.raise("Lone quantifier brackets", None)?;
            }
            let c = char::try_from(cp).unwrap();
            self.raise(&format!("Unexpected character '{c}'"), None)?;
        }
        for name in &self._backreference_names {
            if !self._group_names.contains(name) {
                self.raise("Invalid named capture referenced", None)?;
            }
        }
        self.on_pattern_leave(start, self.index());
        Ok(())
    }

    fn count_capturing_parens(&mut self) -> usize {
        let start = self.index();
        let mut in_class = false;
        let mut escaped = false;
        let mut count = 0;

        while let Some(cp) = self.current_code_point() {
            if escaped {
                escaped = false;
            } else if cp == REVERSE_SOLIDUS {
                escaped = true;
            } else if cp == LEFT_SQUARE_BRACKET {
                in_class = true;
            } else if cp == RIGHT_SQUARE_BRACKET {
                in_class = false;
            } else if cp == LEFT_PARENTHESIS
                && !in_class
                && (self.next_code_point() != Some(QUESTION_MARK)
                    || (self.next_code_point2() == Some(LESS_THAN_SIGN)
                        && self.next_code_point3() != Some(EQUALS_SIGN)
                        && self.next_code_point3() != Some(EXCLAMATION_MARK)))
            {
                count += 1;
            }
            self.advance();
        }

        self.rewind(start);
        count
    }

    fn consume_disjunction(&mut self) -> Result<(), RegExpSyntaxError> {
        let start = self.index();
        let mut i = 0;

        self.on_disjunction_enter(start);
        while {
            self.consume_alternative(i)?;
            i += 1;
            self.eat(VERTICAL_LINE)
        } {}

        if self.consume_quantifier(Some(true)) {
            self.raise("Nothing to repeat", None)?;
        }
        if self.eat(LEFT_CURLY_BRACKET) {
            self.raise("Lone quantifier brackets", None)?;
        }
        self.on_disjunction_leave(start, self.index());
        Ok(())
    }

    fn consume_alternative(&mut self, i: usize) -> Result<(), RegExpSyntaxError> {
        let start = self.index();

        self.on_alternative_enter(start, i);
        while self.current_code_point().is_some() && self.consume_term()? {}
        self.on_alternative_leave(start, self.index(), i);
        Ok(())
    }

    fn consume_term(&mut self) -> Result<bool, RegExpSyntaxError> {
        if self._unicode_mode || self.strict() {
            return Ok(self.consume_assertion()?
                || self.consume_atom()? && self.consume_optional_quantifier());
        }
        Ok(self.consume_assertion()?
            && (!self._last_assertion_is_quantifiable || self.consume_optional_quantifier())
            || self.consume_extended_atom() && self.consume_optional_quantifier())
    }

    fn consume_optional_quantifier(&self) -> bool {
        unimplemented!()
    }

    fn consume_assertion(&mut self) -> Result<bool, RegExpSyntaxError> {
        let start = self.index();
        self._last_assertion_is_quantifiable = false;

        if self.eat(CIRCUMFLEX_ACCENT) {
            self.on_edge_assertion(start, self.index(), AssertionKind::Start);
            return Ok(true);
        }
        if self.eat(DOLLAR_SIGN) {
            self.on_edge_assertion(start, self.index(), AssertionKind::End);
            return Ok(true);
        }
        if self.eat2(REVERSE_SOLIDUS, LATIN_CAPITAL_LETTER_B) {
            self.on_word_boundary_assertion(start, self.index(), AssertionKind::Word, true);
            return Ok(true);
        }
        if self.eat2(REVERSE_SOLIDUS, LATIN_SMALL_LETTER_B) {
            self.on_word_boundary_assertion(start, self.index(), AssertionKind::Word, false);
            return Ok(true);
        }

        if self.eat2(LEFT_PARENTHESIS, QUESTION_MARK) {
            let lookbehind = self.ecma_version() >= EcmaVersion::_2018 && self.eat(LESS_THAN_SIGN);
            let mut negate = false;
            if self.eat(EQUALS_SIGN) || {
                negate = self.eat(EXCLAMATION_MARK);
                negate
            } {
                let kind = if lookbehind {
                    AssertionKind::Lookbehind
                } else {
                    AssertionKind::Lookahead
                };
                self.on_lookaround_assertion_enter(start, kind, negate);
                self.consume_disjunction()?;
                if !self.eat(RIGHT_PARENTHESIS) {
                    self.raise("Unterminated group", None)?;
                }
                self._last_assertion_is_quantifiable = !lookbehind && !self.strict();
                self.on_lookaround_assertion_leave(start, self.index(), kind, negate);
                return Ok(true);
            }
            self.rewind(start);
        }

        Ok(false)
    }

    fn consume_quantifier(&self, no_consume: Option<bool>) -> bool {
        let no_consume = no_consume.unwrap_or_default();
        unimplemented!()
    }

    fn consume_atom(&mut self) -> Result<bool, RegExpSyntaxError> {
        Ok(self.consume_pattern_character()
            || self.consume_dot()
            || self.consume_reverse_solidus_atom_escape()
            || self.consume_character_class()?.is_some()
            || self.consume_uncapturing_group()
            || self.consume_capturing_group())
    }

    fn consume_dot(&mut self) -> bool {
        if self.eat(FULL_STOP) {
            self.on_any_character_set(self.index() - 1, self.index(), CharacterKind::Any);
            return true;
        }
        false
    }

    fn consume_reverse_solidus_atom_escape(&mut self) -> bool {
        let start = self.index();
        if self.eat(REVERSE_SOLIDUS) {
            if self.consume_atom_escape() {
                return true;
            }
            self.rewind(start);
        }
        false
    }

    fn consume_uncapturing_group(&self) -> bool {
        unimplemented!()
    }

    fn consume_capturing_group(&self) -> bool {
        unimplemented!()
    }

    fn consume_extended_atom(&self) -> bool {
        unimplemented!()
    }

    fn consume_pattern_character(&mut self) -> bool {
        let start = self.index();
        let cp = self.current_code_point();
        if let Some(cp) = cp.filter(|&cp| !is_syntax_character(cp)) {
            self.advance();
            self.on_character(start, self.index(), cp);
            return true;
        }
        false
    }

    fn consume_atom_escape(&self) -> bool {
        unimplemented!()
    }

    fn consume_character_class_escape(&self) -> Option<UnicodeSetsConsumeResult> {
        unimplemented!()
    }

    fn consume_character_escape(&self) -> bool {
        unimplemented!()
    }

    fn consume_character_class(
        &mut self,
    ) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
        let start = self.index();
        if self.eat(LEFT_SQUARE_BRACKET) {
            let negate = self.eat(CIRCUMFLEX_ACCENT);
            self.on_character_class_enter(start, negate, self._unicode_sets_mode);
            let result = self.consume_class_contents()?;
            if !self.eat(RIGHT_SQUARE_BRACKET) {
                if self.current_code_point().is_none() {
                    self.raise("Unterminated character class", None)?;
                }
                self.raise("Invalid character in character class", None)?;
            }
            if negate && result.may_contain_strings == Some(true) {
                self.raise("Negated character class may contain strings", None)?;
            }

            self.on_character_class_leave(start, self.index(), negate);

            return Ok(Some(result));
        }
        Ok(None)
    }

    fn consume_class_contents(&mut self) -> Result<UnicodeSetsConsumeResult, RegExpSyntaxError> {
        if self._unicode_sets_mode {
            if self.current_code_point() == Some(RIGHT_SQUARE_BRACKET) {
                return Ok(Default::default());
            }
            let result = self.consume_class_set_expression()?;

            return Ok(result);
        }
        let strict = self.strict() || self._unicode_mode;
        loop {
            let range_start = self.index();
            if !self.consume_class_atom()? {
                break;
            }
            let min = self._last_int_value;

            if !self.eat(HYPHEN_MINUS) {
                continue;
            }
            self.on_character(self.index() - 1, self.index(), HYPHEN_MINUS);

            if !self.consume_class_atom()? {
                break;
            }
            let max = self._last_int_value;

            if min.is_none() || max.is_none() {
                if strict {
                    self.raise("Invalid character class", None)?;
                }
                continue;
            }
            let min = min.unwrap();
            let max = max.unwrap();
            if min > max {
                self.raise("Range out of order in character class", None)?;
            }

            self.on_character_class_range(range_start, self.index(), min, max);
        }

        Ok(Default::default())
    }

    fn consume_class_atom(&mut self) -> Result<bool, RegExpSyntaxError> {
        let start = self.index();
        let cp = self.current_code_point();

        if let Some(cp) = cp.filter(|&cp| cp != REVERSE_SOLIDUS && cp != RIGHT_SQUARE_BRACKET) {
            self.advance();
            self._last_int_value = Some(cp);
            self.on_character(start, self.index(), self._last_int_value.unwrap());
            return Ok(true);
        }

        if self.eat(REVERSE_SOLIDUS) {
            if self.consume_class_escape() {
                return Ok(true);
            }
            if !self.strict() && self.current_code_point() == Some(LATIN_SMALL_LETTER_C) {
                self._last_int_value = Some(REVERSE_SOLIDUS);
                self.on_character(start, self.index(), self._last_int_value.unwrap());
                return Ok(true);
            }
            if self.strict() || self._unicode_mode {
                self.raise("Invalid escape", None)?;
            }
            self.rewind(start);
        }

        Ok(false)
    }

    fn consume_class_escape(&self) -> bool {
        unimplemented!()
    }

    fn consume_class_set_expression(
        &mut self,
    ) -> Result<UnicodeSetsConsumeResult, RegExpSyntaxError> {
        let start = self.index();
        let mut may_contain_strings = Some(false);
        let mut result: Option<UnicodeSetsConsumeResult> = Default::default();
        if self.consume_class_set_character() {
            if self.consume_class_set_range_from_operator(start)? {
                self.consume_class_union_right(Default::default())?;
                return Ok(Default::default());
            }
            may_contain_strings = Some(false);
        } else if let Some(result) = {
            result = self.consume_class_set_operand()?;
            result
        } {
            may_contain_strings = result.may_contain_strings;
        } else {
            let cp = self.current_code_point();
            if cp == Some(REVERSE_SOLIDUS) {
                self.advance();
                self.raise("Invalid escape", None)?;
            }
            if cp == self.next_code_point() && is_class_set_reserved_double_punctuator_character(cp)
            {
                self.raise("Invalid set operation in character class", None)?;
            }
            self.raise("Invalid character in character class", None)?;
        }

        if self.eat2(AMPERSAND, AMPERSAND) {
            while self.current_code_point() != Some(AMPERSAND) && {
                result = self.consume_class_set_operand()?;
                result.is_some()
            } {
                self.on_class_intersection(start, self.index());
                if result.unwrap().may_contain_strings != Some(true) {
                    may_contain_strings = Some(false);
                }
                if self.eat2(AMPERSAND, AMPERSAND) {
                    continue;
                }

                return Ok(UnicodeSetsConsumeResult {
                    may_contain_strings,
                });
            }

            self.raise("Invalid character in character class", None)?;
        }
        if self.eat2(HYPHEN_MINUS, HYPHEN_MINUS) {
            while self.consume_class_set_operand()?.is_some() {
                self.on_class_subtraction(start, self.index());
                if self.eat2(HYPHEN_MINUS, HYPHEN_MINUS) {
                    continue;
                }

                return Ok(UnicodeSetsConsumeResult {
                    may_contain_strings,
                });
            }
            self.raise("Invalid character in character class", None)?;
        }
        self.consume_class_union_right(UnicodeSetsConsumeResult {
            may_contain_strings,
        })
    }

    fn consume_class_union_right(
        &mut self,
        left_result: UnicodeSetsConsumeResult,
    ) -> Result<UnicodeSetsConsumeResult, RegExpSyntaxError> {
        let mut may_contain_strings = left_result.may_contain_strings;
        loop {
            let start = self.index();
            if self.consume_class_set_character() {
                self.consume_class_set_range_from_operator(start)?;
                continue;
            }
            let result = self.consume_class_set_operand()?;
            if let Some(result) = result {
                if result.may_contain_strings == Some(true) {
                    may_contain_strings = Some(true);
                }
                continue;
            }
            break;
        }

        Ok(UnicodeSetsConsumeResult {
            may_contain_strings,
        })
    }

    fn consume_class_set_range_from_operator(
        &mut self,
        start: usize,
    ) -> Result<bool, RegExpSyntaxError> {
        let current_start = self.index();
        let min = self._last_int_value;
        if self.eat(HYPHEN_MINUS) {
            if self.consume_class_set_character() {
                let max = self._last_int_value;

                if min.is_none() || max.is_none() {
                    self.raise("Invalid character class", None)?;
                }
                let min = min.unwrap();
                let max = max.unwrap();
                if min > max {
                    self.raise("Range out of order in character class", None)?;
                }
                self.on_character_class_range(start, self.index(), min, max);
                return Ok(true);
            }
            self.rewind(current_start);
        }
        Ok(false)
    }

    fn consume_class_set_operand(
        &mut self,
    ) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
        let mut result: Option<UnicodeSetsConsumeResult>;
        result = self.consume_nested_class()?;
        if let Some(result) = result {
            return Ok(Some(result));
        }
        result = self.consume_class_string_disjunction()?;
        if let Some(result) = result {
            return Ok(Some(result));
        }
        if self.consume_class_set_character() {
            return Ok(Some(Default::default()));
        }
        Ok(None)
    }

    fn consume_nested_class(
        &mut self,
    ) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
        let start = self.index();
        if self.eat(LEFT_SQUARE_BRACKET) {
            let negate = self.eat(CIRCUMFLEX_ACCENT);
            self.on_character_class_enter(start, negate, true);
            let result = self.consume_class_contents()?;
            if !self.eat(RIGHT_SQUARE_BRACKET) {
                self.raise("Unterminated character class", None)?;
            }
            if negate && result.may_contain_strings == Some(true) {
                self.raise("Negated character class may contain strings", None)?;
            }
            self.on_character_class_leave(start, self.index(), negate);

            return Ok(Some(result));
        }
        if self.eat(REVERSE_SOLIDUS) {
            let result = self.consume_character_class_escape();
            if let Some(result) = result {
                return Ok(Some(result));
            }
            self.rewind(start);
        }
        Ok(None)
    }

    fn consume_class_string_disjunction(
        &mut self,
    ) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
        let start = self.index();
        if self.eat3(REVERSE_SOLIDUS, LATIN_SMALL_LETTER_Q, LEFT_CURLY_BRACKET) {
            self.on_class_string_disjunction_enter(start);

            let mut i = 0;
            let mut may_contain_strings = false;
            while {
                if self.consume_class_string(i).may_contain_strings == Some(true) {
                    may_contain_strings = true;
                }
                i += 1;
                self.eat(VERTICAL_LINE)
            } {}

            if self.eat(RIGHT_CURLY_BRACKET) {
                self.on_class_string_disjunction_leave(start, self.index());

                return Ok(Some(UnicodeSetsConsumeResult {
                    may_contain_strings: Some(may_contain_strings),
                }));
            }
            self.raise("Unterminated class string disjunction", None)?;
        }
        Ok(None)
    }

    fn consume_class_string(&self, i: usize) -> UnicodeSetsConsumeResult {
        unimplemented!()
    }

    fn consume_class_set_character(&mut self) -> bool {
        let start = self.index();
        let cp = self.current_code_point();
        if cp != self.next_code_point() || !is_class_set_reserved_double_punctuator_character(cp) {
            if let Some(cp) = cp.filter(|&cp| !is_class_set_syntax_character(cp)) {
                self._last_int_value = Some(cp);
                self.advance();
                self.on_character(start, self.index(), self._last_int_value.unwrap());
                return true;
            }
        }
        if self.eat(REVERSE_SOLIDUS) {
            if self.consume_character_escape() {
                return true;
            }
            if is_class_set_reserved_punctuator(self.current_code_point()) {
                self._last_int_value = self.current_code_point();
                self.advance();
                self.on_character(start, self.index(), self._last_int_value.unwrap());
                return true;
            }
            if self.eat(LATIN_SMALL_LETTER_B) {
                self._last_int_value = Some(BACKSPACE);
                self.on_character(start, self.index(), self._last_int_value.unwrap());
                return true;
            }
            self.rewind(start);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use speculoos::prelude::*;

    use crate::ast::str_to_wtf_16;

    use super::*;

    fn validator<'a>() -> RegExpValidator<'a> {
        RegExpValidator::new(None)
    }

    fn get_error_for_pattern(
        source: &str,
        start: usize,
        end: usize,
        flags: ValidatePatternFlags,
    ) -> RegExpSyntaxError {
        validator()
            .validate_pattern(&str_to_wtf_16(source), Some(start), Some(end), Some(flags))
            .expect_err("Should fail, but succeeded.")
    }

    fn get_error_for_flags(source: &str, start: usize, end: usize) -> RegExpSyntaxError {
        validator()
            .validate_flags(&str_to_wtf_16(source), Some(start), Some(end))
            .expect_err("Should fail, but succeeded.")
    }

    fn get_error_for_literal(source: &str, start: usize, end: usize) -> RegExpSyntaxError {
        validator()
            .validate_literal(&str_to_wtf_16(source), Some(start), Some(end))
            .expect_err("Should fail, but succeeded.")
    }

    #[test]
    fn test_validate_pattern_should_throw_syntax_error() {
        #[derive(Deserialize)]
        struct Case {
            source: String,
            start: usize,
            end: usize,
            flags: ValidatePatternFlags,
            error: RegExpSyntaxError,
        }

        let cases: Vec<Case> = serde_json::from_value(json!([
            {
                "source": "abcd",
                "start": 0,
                "end": 2,
                "flags": { "unicode": true, "unicodeSets": true },
                "error": {
                    "message":
                        "Invalid regular expression: /ab/uv: Invalid regular expression flags",
                    "index": 3,
                },
            },
            {
                "source": "[A]",
                "start": 0,
                "end": 2,
                "flags": { "unicode": true, "unicodeSets": false },
                "error": {
                    "message":
                        "Invalid regular expression: /[A/u: Unterminated character class",
                    "index": 2,
                },
            },
            {
                "source": "[[A]]",
                "start": 0,
                "end": 4,
                "flags": { "unicode": false, "unicodeSets": true },
                "error": {
                    "message":
                        "Invalid regular expression: /[[A]/v: Unterminated character class",
                    "index": 4,
                },
            },
            {
                "source": " /[[A]/v ",
                "start": 2,
                "end": 6,
                "flags": { "unicode": false, "unicodeSets": true },
                "error": {
                    "message":
                        "Invalid regular expression: /[[A]/v: Unterminated character class",
                    "index": 6,
                },
            },
        ]))
        .unwrap();

        for test in cases {
            let error = get_error_for_pattern(&test.source, test.start, test.end, test.flags);

            assert_that!(&error).is_equal_to(&test.error);
        }
    }

    #[test]
    fn test_validate_flags_should_throw_syntax_error() {
        #[derive(Deserialize)]
        struct Case {
            source: String,
            start: usize,
            end: usize,
            error: RegExpSyntaxError,
        }

        let cases: Vec<Case> = serde_json::from_value(json!([
            {
                "source": "abcd",
                "start": 0,
                "end": 2,
                "error": {
                    "message": "Invalid regular expression: Invalid flag 'a'",
                    "index": 0,
                },
            },
            {
                "source": "dd",
                "start": 0,
                "end": 2,
                "error": {
                    "message": "Invalid regular expression: Duplicated flag 'd'",
                    "index": 0,
                },
            },
            {
                "source": "/a/dd",
                "start": 3,
                "end": 5,
                "error": {
                    "message": "Invalid regular expression: Duplicated flag 'd'",
                    "index": 3,
                },
            },
        ]))
        .unwrap();

        for test in cases {
            let error = get_error_for_flags(&test.source, test.start, test.end);

            assert_that!(&error).is_equal_to(&test.error);
        }
    }

    #[test]
    fn test_validate_literal_should_throw_syntax_error() {
        #[derive(Deserialize)]
        struct Case {
            source: String,
            start: usize,
            end: usize,
            error: RegExpSyntaxError,
        }

        let cases: Vec<Case> = serde_json::from_value(json!([
            {
                "source": " /[/ ",
                "start": 1,
                "end": 4,
                "error": {
                    "message":
                        "Invalid regular expression: /[/: Unterminated character class",
                    "index": 4,
                },
            },
        ]))
        .unwrap();

        for test in cases {
            let error = get_error_for_literal(&test.source, test.start, test.end);

            assert_that!(&error).is_equal_to(&test.error);
        }
    }
}
