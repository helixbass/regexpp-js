use crate::EcmaVersion;

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
    on_character_class_range: Option<Box<dyn FnMut(usize, usize, UnicodeCodePoint, UnicodeCodePoint)>>,
    on_class_intersection: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_subtraction: Option<Box<dyn FnMut(usize, usize)>>,
    on_class_string_disjunction_enter: Option<Box<dyn FnMut(usize)>>,
    on_class_string_disjunction_leave: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_enter: Option<Box<dyn FnMut(usize, usize)>>,
    on_string_alternative_leave: Option<Box<dyn FnMut(usize, usize, usize)>>,
}

pub struct RegExpValidator;

impl RegExpValidator {
    pub fn new(options: Options) -> Self {
        unimplemented!()
    }
}
