use serde::Deserialize;

use crate::{EcmaVersion, RegExpSyntaxError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RegExpValidatorSourceContextKind {
    Flags,
    Literal,
    Pattern,
}

struct RegExpValidatorSourceContext<'a> {
    source: &'a str,
    start: usize,
    end: usize,
    kind: RegExpValidatorSourceContextKind,
}

pub struct RegExpFlags {
    global: bool,
    ignore_case: bool,
    multiline: bool,
    unicode: bool,
    sticky: bool,
    dot_all: bool,
    has_indices: bool,
    unicode_sets: bool,
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

pub type UnicodeCodePoint = u32;

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
    on_character: Option<Box<dyn FnMut(usize, usize, UnicodeCodePoint)>>,
    on_backreference: Option<Box<dyn FnMut(usize, usize, CapturingGroupKey)>>,
    on_character_class_enter: Option<Box<dyn FnMut(usize, bool, bool)>>,
    on_character_class_leave: Option<Box<dyn FnMut(usize, usize, bool)>>,
    on_character_class_range:
        Option<Box<dyn FnMut(usize, usize, UnicodeCodePoint, UnicodeCodePoint)>>,
    on_class_intersection: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_subtraction: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_string_disjunction_enter: Option<Box<dyn FnMut(usize)>>,
    on_class_string_disjunction_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_enter: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_leave: Option<Box<dyn FnMut(usize, usize, usize)>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ValidatePatternFlags {
    unicode: Option<bool>,
    unicode_sets: Option<bool>,
}

pub struct RegExpValidator<'a> {
    _options: Options,
    _src_ctx: Option<RegExpValidatorSourceContext<'a>>,
}

impl<'a> RegExpValidator<'a> {
    pub fn new(options: Option<Options>) -> Self {
        Self {
            _options: options.unwrap_or_default(),
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
        source: &str,
        start: usize,
        end: usize,
        flags: Option<ValidatePatternFlags>,
    ) -> Result<(), RegExpSyntaxError> {
        unimplemented!()
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
