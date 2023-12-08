use std::collections::HashSet;

use once_cell::sync::Lazy;
use serde::Deserialize;
use squalid::OptionExt;

use crate::{
    ecma_versions::LATEST_ECMA_VERSION,
    reader::CodePoint,
    regexp_syntax_error::{self, new_reg_exp_syntax_error},
    unicode::{
        AMPERSAND, ASTERISK, BACKSPACE, CIRCUMFLEX_ACCENT, COLON, COMMA, COMMERCIAL_AT,
        DOLLAR_SIGN, EQUALS_SIGN, EXCLAMATION_MARK, FULL_STOP, GRAVE_ACCENT, GREATER_THAN_SIGN,
        HYPHEN_MINUS, LATIN_CAPITAL_LETTER_B, LATIN_SMALL_LETTER_B, LATIN_SMALL_LETTER_C,
        LEFT_CURLY_BRACKET, LEFT_PARENTHESIS, LEFT_SQUARE_BRACKET, LESS_THAN_SIGN, NUMBER_SIGN,
        PERCENT_SIGN, PLUS_SIGN, QUESTION_MARK, REVERSE_SOLIDUS, RIGHT_CURLY_BRACKET,
        RIGHT_PARENTHESIS, RIGHT_SQUARE_BRACKET, SEMICOLON, SOLIDUS, TILDE, VERTICAL_LINE,
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

pub struct RegExpValidatorSourceContext<'a> {
    pub source: &'a str,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LookaroundKind {
    Lookahead,
    Lookbehind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    End,
    Start,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WordBoundaryKind {
    Word,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnyCharacterKind {
    Any,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EscapeCharacterKind {
    Digit,
    Space,
    Word,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnicodePropertyCharacterKind {
    Property,
}

pub enum CapturingGroupKey<'a> {
    Index(usize),
    Name(&'a str),
}

#[derive(Default)]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
    on_literal_enter: Option<Box<dyn FnMut(usize)>>,
    on_literal_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_reg_exp_flags: Option<Box<dyn FnMut(usize, usize, RegExpFlags)>>,
    on_pattern_enter: Option<Box<dyn FnMut(usize)>>,
    on_pattern_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_disjunction_enter: Option<Box<dyn FnMut(usize)>>,
    on_disjunction_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_alternative_enter: Option<Box<dyn FnMut(usize, usize)>>,
    on_alternative_leave: Option<Box<dyn FnMut(usize, usize, usize)>>,
    on_group_enter: Option<Box<dyn FnMut(usize)>>,
    on_group_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_capturing_group_enter: Option<Box<dyn FnMut(usize, Option<&str>)>>,
    on_capturing_group_leave: Option<Box<dyn FnMut(usize, usize, Option<&str>)>>,
    on_quantifier: Option<Box<dyn FnMut(usize, usize, usize, usize, bool)>>,
    on_lookaround_assertion_enter: Option<Box<dyn FnMut(usize, LookaroundKind, bool)>>,
    on_lookaround_assertion_leave: Option<Box<dyn FnMut(usize, usize, LookaroundKind, bool)>>,
    on_edge_assertion: Option<Box<dyn FnMut(usize, usize, EdgeKind)>>,
    on_word_boundary_assertion: Option<Box<dyn FnMut(usize, usize, WordBoundaryKind, bool)>>,
    on_any_character_set: Option<Box<dyn FnMut(usize, usize, AnyCharacterKind)>>,
    on_escape_character_set: Option<Box<dyn FnMut(usize, usize, EscapeCharacterKind, bool)>>,
    on_unicode_property_character_set: Option<
        Box<dyn FnMut(usize, usize, UnicodePropertyCharacterKind, &str, Option<&str>, bool, bool)>,
    >,
    on_character: Option<Box<dyn FnMut(usize, usize, CodePoint)>>,
    on_backreference: Option<Box<dyn FnMut(usize, usize, CapturingGroupKey)>>,
    on_character_class_enter: Option<Box<dyn FnMut(usize, bool, bool)>>,
    on_character_class_leave: Option<Box<dyn FnMut(usize, usize, bool)>>,
    on_character_class_range: Option<Box<dyn FnMut(usize, usize, CodePoint, CodePoint)>>,
    on_class_intersection: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_subtraction: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_string_disjunction_enter: Option<Box<dyn FnMut(usize)>>,
    on_class_string_disjunction_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_enter: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_leave: Option<Box<dyn FnMut(usize, usize, usize)>>,
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

#[derive(Copy, Clone, Default)]
struct RaiseContext {
    index: Option<usize>,
    unicode: Option<bool>,
    unicode_sets: Option<bool>,
}

pub struct RegExpValidator<'a> {
    _options: Options,
    _reader: Reader,
    _unicode_mode: bool,
    _unicode_sets_mode: bool,
    _n_flag: bool,
    _last_int_value: Option<CodePoint>,
    _last_assertion_is_quantifiable: bool,
    _num_capturing_parens: usize,
    _group_names: HashSet<String>,
    _backreference_names: HashSet<String>,
    _src_ctx: Option<RegExpValidatorSourceContext<'a>>,
}

impl<'a> RegExpValidator<'a> {
    pub fn new(options: Option<Options>) -> Self {
        Self {
            _options: options.unwrap_or_default(),
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

    pub fn validate_pattern(
        &mut self,
        source: &'a str,
        start: Option<usize>,
        end: Option<usize>,
        flags: Option<ValidatePatternFlags>,
    ) -> Result<(), RegExpSyntaxError> {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(source.len());
        self._src_ctx = Some(RegExpValidatorSourceContext {
            source,
            start,
            end,
            kind: RegExpValidatorSourceContextKind::Pattern,
        });
        self.validate_pattern_internal(source, start, end, flags)
    }

    fn validate_pattern_internal(
        &mut self,
        source: &'a str,
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

    fn strict(&self) -> bool {
        self._options.strict.unwrap_or_default() || self._unicode_mode
    }

    fn ecma_version(&self) -> EcmaVersion {
        self._options.ecma_version.unwrap_or(LATEST_ECMA_VERSION)
    }

    fn on_pattern_enter(&mut self, start: usize) {
        if let Some(on_pattern_enter) = self._options.on_pattern_enter.as_mut() {
            on_pattern_enter(start);
        }
    }

    fn on_pattern_leave(&mut self, start: usize, end: usize) {
        if let Some(on_pattern_leave) = self._options.on_pattern_leave.as_mut() {
            on_pattern_leave(start, end);
        }
    }

    fn on_disjunction_enter(&mut self, start: usize) {
        if let Some(on_disjunction_enter) = self._options.on_disjunction_enter.as_mut() {
            on_disjunction_enter(start);
        }
    }

    fn on_disjunction_leave(&mut self, start: usize, end: usize) {
        if let Some(on_disjunction_leave) = self._options.on_disjunction_leave.as_mut() {
            on_disjunction_leave(start, end);
        }
    }

    fn on_alternative_enter(&mut self, start: usize, index: usize) {
        if let Some(on_alternative_enter) = self._options.on_alternative_enter.as_mut() {
            on_alternative_enter(start, index);
        }
    }

    fn on_alternative_leave(&mut self, start: usize, end: usize, index: usize) {
        if let Some(on_alternative_leave) = self._options.on_alternative_leave.as_mut() {
            on_alternative_leave(start, end, index);
        }
    }

    fn on_lookaround_assertion_enter(&mut self, start: usize, kind: LookaroundKind, negate: bool) {
        if let Some(on_lookaround_assertion_enter) =
            self._options.on_lookaround_assertion_enter.as_mut()
        {
            on_lookaround_assertion_enter(start, kind, negate);
        }
    }

    fn on_lookaround_assertion_leave(
        &mut self,
        start: usize,
        end: usize,
        kind: LookaroundKind,
        negate: bool,
    ) {
        if let Some(on_lookaround_assertion_leave) =
            self._options.on_lookaround_assertion_leave.as_mut()
        {
            on_lookaround_assertion_leave(start, end, kind, negate);
        }
    }

    fn on_edge_assertion(&mut self, start: usize, end: usize, kind: EdgeKind) {
        if let Some(on_edge_assertion) = self._options.on_edge_assertion.as_mut() {
            on_edge_assertion(start, end, kind);
        }
    }

    fn on_word_boundary_assertion(
        &mut self,
        start: usize,
        end: usize,
        kind: WordBoundaryKind,
        negate: bool,
    ) {
        if let Some(on_word_boundary_assertion) = self._options.on_word_boundary_assertion.as_mut()
        {
            on_word_boundary_assertion(start, end, kind, negate);
        }
    }

    fn on_any_character_set(&mut self, start: usize, end: usize, kind: AnyCharacterKind) {
        if let Some(on_any_character_set) = self._options.on_any_character_set.as_mut() {
            on_any_character_set(start, end, kind);
        }
    }

    fn on_character(&mut self, start: usize, end: usize, value: CodePoint) {
        if let Some(on_character) = self._options.on_character.as_mut() {
            on_character(start, end, value);
        }
    }

    fn on_character_class_enter(&mut self, start: usize, negate: bool, unicode_sets: bool) {
        if let Some(on_character_class_enter) = self._options.on_character_class_enter.as_mut() {
            on_character_class_enter(start, negate, unicode_sets);
        }
    }

    fn on_character_class_leave(&mut self, start: usize, end: usize, negate: bool) {
        if let Some(on_character_class_leave) = self._options.on_character_class_leave.as_mut() {
            on_character_class_leave(start, end, negate);
        }
    }

    fn on_character_class_range(
        &mut self,
        start: usize,
        end: usize,
        min: CodePoint,
        max: CodePoint,
    ) {
        if let Some(on_character_class_range) = self._options.on_character_class_range.as_mut() {
            on_character_class_range(start, end, min, max);
        }
    }

    fn on_class_intersection(&mut self, start: usize, end: usize) {
        if let Some(on_class_intersection) = self._options.on_class_intersection.as_mut() {
            on_class_intersection(start, end);
        }
    }

    fn on_class_subtraction(&mut self, start: usize, end: usize) {
        if let Some(on_class_subtraction) = self._options.on_class_subtraction.as_mut() {
            on_class_subtraction(start, end);
        }
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
            || self._options.strict == Some(true) && self.ecma_version() >= EcmaVersion::_2023;
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

    fn reset(&mut self, source: &'a str, start: usize, end: usize) {
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
            self.on_edge_assertion(start, self.index(), EdgeKind::Start);
            return Ok(true);
        }
        if self.eat(DOLLAR_SIGN) {
            self.on_edge_assertion(start, self.index(), EdgeKind::End);
            return Ok(true);
        }
        if self.eat2(REVERSE_SOLIDUS, LATIN_CAPITAL_LETTER_B) {
            self.on_word_boundary_assertion(start, self.index(), WordBoundaryKind::Word, true);
            return Ok(true);
        }
        if self.eat2(REVERSE_SOLIDUS, LATIN_SMALL_LETTER_B) {
            self.on_word_boundary_assertion(start, self.index(), WordBoundaryKind::Word, false);
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
                    LookaroundKind::Lookbehind
                } else {
                    LookaroundKind::Lookahead
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
            self.on_any_character_set(self.index() - 1, self.index(), AnyCharacterKind::Any);
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
            may_contain_strings
        })
    }

    fn consume_class_set_range_from_operator(&mut self, start: usize) -> Result<bool, RegExpSyntaxError> {
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

    fn consume_class_set_operand(&mut self) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
        let mut result: Option<UnicodeSetsConsumeResult>;
        result = self.consume_nested_class()?;
        if let Some(result) = result {
            return Ok(Some(result));
        }
        result = self.consume_class_string_disjunction();
        if let Some(result) = result {
            return Ok(Some(result));
        }
        if self.consume_class_set_character() {
            return Ok(Some(Default::default()));
        }
        Ok(None)
    }

    fn consume_nested_class(&mut self) -> Result<Option<UnicodeSetsConsumeResult>, RegExpSyntaxError> {
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

    fn consume_class_string_disjunction(&self) -> Option<UnicodeSetsConsumeResult> {
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
            .validate_pattern(source, Some(start), Some(end), Some(flags))
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
}
