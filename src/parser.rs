use std::rc::Rc;

use id_arena::Id;
use serde::Deserialize;

use crate::{arena::AllArenas, ecma_versions::EcmaVersion, validator, RegExpValidator, ast::Node, Result};

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
}

struct RegExpParserState {}

impl RegExpParserState {
    pub fn new(options: Option<Options>) -> Self {
        unimplemented!()
    }
}

impl validator::Options for RegExpParserState {
    fn strict(&self) -> Option<bool> {
        todo!()
    }

    fn ecma_version(&self) -> Option<EcmaVersion> {
        todo!()
    }

    fn on_literal_enter(&self, start: usize) {}

    fn on_literal_leave(&self, start: usize, end: usize) {}

    fn on_reg_exp_flags(&self, start: usize, end: usize, flags: validator::RegExpFlags) {}

    fn on_pattern_enter(&self, start: usize) {}

    fn on_pattern_leave(&self, start: usize, end: usize) {}

    fn on_disjunction_enter(&self, start: usize) {}

    fn on_disjunction_leave(&self, start: usize, end: usize) {}

    fn on_alternative_enter(&self, start: usize, index: usize) {}

    fn on_alternative_leave(&self, start: usize, end: usize, index: usize) {}

    fn on_group_enter(&self, start: usize) {}

    fn on_group_leave(&self, start: usize, end: usize) {}

    fn on_capturing_group_enter(&self, start: usize, TODO: Option<&str>) {}

    fn on_capturing_group_leave(&self, start: usize, end: usize, TODO: Option<&str>) {}

    fn on_quantifier(&self, start: usize, TODO: usize, a: usize, b: usize, c: bool) {}

    fn on_lookaround_assertion_enter(
        &self,
        start: usize,
        kind: validator::AssertionKind,
        negate: bool,
    ) {
    }

    fn on_lookaround_assertion_leave(
        &self,
        start: usize,
        end: usize,
        kind: validator::AssertionKind,
        negate: bool,
    ) {
    }

    fn on_edge_assertion(&self, start: usize, end: usize, kind: validator::AssertionKind) {}

    fn on_word_boundary_assertion(
        &self,
        start: usize,
        end: usize,
        kind: validator::AssertionKind,
        negate: bool,
    ) {
    }

    fn on_any_character_set(&self, start: usize, end: usize, kind: validator::CharacterKind) {}

    fn on_escape_character_set(
        &self,
        start: usize,
        TODO: usize,
        kind: validator::CharacterKind,
        b: bool,
    ) {
    }

    fn on_unicode_property_character_set(
        &self,
        start: usize,
        TODO: usize,
        kind: validator::CharacterKind,
        a: &str,
        b: Option<&str>,
        c: bool,
        d: bool,
    ) {
    }

    fn on_character(&self, start: usize, end: usize, value: crate::CodePoint) {}

    fn on_backreference(&self, start: usize, TODO: usize, a: validator::CapturingGroupKey) {}

    fn on_character_class_enter(&self, start: usize, negate: bool, unicode_sets: bool) {}

    fn on_character_class_leave(&self, start: usize, end: usize, negate: bool) {}

    fn on_character_class_range(
        &self,
        start: usize,
        end: usize,
        min: crate::CodePoint,
        max: crate::CodePoint,
    ) {
    }

    fn on_class_intersection(&self, start: usize, end: usize) {}

    fn on_class_subtraction(&self, start: usize, end: usize) {}

    fn on_class_string_disjunction_enter(&self, start: usize) {}

    fn on_class_string_disjunction_leave(&self, start: usize, end: usize) {}

    fn on_string_alternative_enter(&self, start: usize, TODO: usize) {}

    fn on_string_alternative_leave(&self, start: usize, TODO: usize, a: usize) {}
}

pub struct RegExpParser<'a, 'b> {
    _arena: &'b mut AllArenas<'a>,
    _state: Rc<RegExpParserState>,
    _validator: RegExpValidator<'a>,
}

impl<'a, 'b> RegExpParser<'a, 'b> {
    pub fn new(arena: &'b mut AllArenas<'a>, options: Option<Options>) -> Self {
        let state = Rc::new(RegExpParserState::new(options));
        Self {
            _arena: arena,
            _state: state.clone(),
            _validator: RegExpValidator::new(Some(state)),
        }
    }

    pub fn parse_literal(
        &mut self,
        source: &'a str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Id<Node<'a>>> {
        unimplemented!()
    }
}
