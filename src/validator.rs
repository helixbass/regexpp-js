use std::collections::HashSet;

use serde::Deserialize;

use crate::{EcmaVersion, RegExpSyntaxError, Reader, ecma_versions::LATEST_ECMA_VERSION, regexp_syntax_error::{new_reg_exp_syntax_error, self}};

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
    _group_names: HashSet<String>,
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
            _group_names: Default::default(),
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
        let mode = self._parse_flags_option_to_mode(
            flags,
            end,
        )?;

        self._unicode_mode = mode.unicode_mode;
        self._n_flag = mode.n_flag;
        self._unicode_sets_mode = mode.unicode_sets_mode;
        self.reset(source, start, end);
        self.consume_pattern();

        if !self._n_flag &&
            self.ecma_version() >= EcmaVersion::_2018 &&
            !self._group_names.is_empty() {
            self._n_flag = true;
            self.rewind(start);
        }

        Ok(())
    }

    fn ecma_version(
        &self,
    ) -> EcmaVersion {
        self._options.ecma_version.unwrap_or(LATEST_ECMA_VERSION)
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
                })
            )?;
        }

        let unicode_mode = unicode || unicode_sets;
        let n_flag = unicode && self.ecma_version() >= EcmaVersion::_2018 ||
            unicode_sets ||
            self._options.strict == Some(true) && self.ecma_version() >= EcmaVersion::_2023;
        let unicode_sets_mode = unicode_sets;

        Ok(Mode {
            unicode_mode,
            n_flag,
            unicode_sets_mode
        })
    }

    fn index(
        &self,
    ) -> usize {
        self._reader.index()
    }

    fn reset(
        &mut self,
        source: &str,
        start: usize,
        end: usize,
    ) {
        self._reader.reset(source, start, end, self._unicode_mode);
    }

    fn rewind(
        &mut self,
        index: usize,
    ) {
        self._reader.rewind(index);
    }

    fn raise(
        &self,
        message: &str,
        context: Option<RaiseContext>,
    ) -> Result<(), RegExpSyntaxError> {
        Err(new_reg_exp_syntax_error(
            self._src_ctx.as_ref().unwrap(),
            regexp_syntax_error::Flags {
                unicode: context.and_then(|context| context.unicode).unwrap_or(self._unicode_mode && !self._unicode_sets_mode),
                unicode_sets: context.and_then(|context| context.unicode_sets).unwrap_or(self._unicode_sets_mode),
            },
            context.and_then(|context| context.index).unwrap_or(self.index()),
            message,
        ))
    }

    fn consume_pattern(
        &self,
    ) {
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
