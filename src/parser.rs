use std::{rc::Rc, collections::HashMap, cell::RefCell};

use id_arena::Id;
use serde::Deserialize;

use crate::{arena::AllArenas, ecma_versions::{EcmaVersion, LATEST_ECMA_VERSION}, validator::{self, RegExpFlags}, RegExpValidator, ast::{Node, Flags, NodeBase}, Result};

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
}

struct RegExpParserState<'a> {
    _arena: &'a AllArenas,
    strict: bool,
    ecma_version: EcmaVersion,
    _node: Option<Id<Node> /*AppendableNode*/>,
    _expression_buffer_map: HashMap<Id<Node>, Id<Node>>,
    _flags: RefCell<Option<Id<Node> /*Flags*/>>,
    _backreferences: Vec<Id<Node> /*Backreference*/>,
    _capturing_groups: Vec<Id<Node> /*CapturingGroup*/>,
    source: Vec<u16>,
}

impl<'a> RegExpParserState<'a> {
    pub fn new(
        arena: &'a AllArenas,
        options: Option<Options>) -> Self {
        Self {
            _arena: arena,
            strict: options.and_then(|options| options.strict).unwrap_or_default(),
            ecma_version: options.and_then(|options| options.ecma_version).unwrap_or(LATEST_ECMA_VERSION),
            _node: Default::default(),
            _expression_buffer_map: Default::default(),
            _flags: Default::default(),
            _backreferences: Default::default(),
            _capturing_groups: Default::default(),
            source: Default::default(),
        }
    }

    fn pattern(&self) -> Id<Node> {
        self._node.unwrap()
    }

    fn flags(&self) -> Id<Node> {
        self._flags.borrow().unwrap()
    }
}

impl<'a> validator::Options for RegExpParserState<'a> {
    fn strict(&self) -> Option<bool> {
        Some(self.strict)
    }

    fn ecma_version(&self) -> Option<EcmaVersion> {
        Some(self.ecma_version)
    }

    fn on_literal_enter(&self, start: usize) {}

    fn on_literal_leave(&self, start: usize, end: usize) {}

    fn on_reg_exp_flags(&self, start: usize, end: usize, flags: RegExpFlags) {
        *self._flags.borrow_mut() = Some(self._arena.alloc_node(Node::new_flags(
            None,
            start,
            end,
            self.source[start..end].to_owned(),
            flags.dot_all,
            flags.global,
            flags.has_indices,
            flags.ignore_case,
            flags.multiline,
            flags.sticky,
            flags.unicode,
            flags.unicode_sets,
        )));
    }

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

pub struct RegExpParser<'a> {
    _arena: &'a AllArenas,
    _state: Rc<RegExpParserState<'a>>,
    _validator: RegExpValidator<'a>,
}

impl<'a> RegExpParser<'a> {
    pub fn new(arena: &'a AllArenas, options: Option<Options>) -> Self {
        let state = Rc::new(RegExpParserState::new(arena, options));
        Self {
            _arena: arena,
            _state: state.clone(),
            _validator: RegExpValidator::new(Some(state)),
        }
    }

    pub fn parse_literal(
        &mut self,
        source: &str,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<Id<Node>> {
        unimplemented!()
    }
}
