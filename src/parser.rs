use std::{cell::RefCell, collections::HashMap, rc::Rc};

use id_arena::Id;
use serde::Deserialize;
use squalid::EverythingExt;

use crate::{
    arena::AllArenas,
    ast::{Flags, Node, NodeBase, NodeInterface},
    ecma_versions::{EcmaVersion, LATEST_ECMA_VERSION},
    validator::{self, AssertionKind, CapturingGroupKey, RegExpFlags},
    RegExpValidator, Result,
};

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
    _node: RefCell<Option<Id<Node> /*AppendableNode*/>>,
    _expression_buffer_map: HashMap<Id<Node>, Id<Node>>,
    _flags: RefCell<Option<Id<Node> /*Flags*/>>,
    _backreferences: RefCell<Vec<Id<Node> /*Backreference*/>>,
    _capturing_groups: RefCell<Vec<Id<Node> /*CapturingGroup*/>>,
    source: Vec<u16>,
}

impl<'a> RegExpParserState<'a> {
    pub fn new(arena: &'a AllArenas, options: Option<Options>) -> Self {
        Self {
            _arena: arena,
            strict: options
                .and_then(|options| options.strict)
                .unwrap_or_default(),
            ecma_version: options
                .and_then(|options| options.ecma_version)
                .unwrap_or(LATEST_ECMA_VERSION),
            _node: Default::default(),
            _expression_buffer_map: Default::default(),
            _flags: Default::default(),
            _backreferences: Default::default(),
            _capturing_groups: Default::default(),
            source: Default::default(),
        }
    }

    fn pattern(&self) -> Id<Node> {
        self._node.borrow().unwrap()
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

    fn on_pattern_enter(&self, start: usize) {
        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_pattern(
            None,
            start,
            start,
            Default::default(),
            Default::default(),
        )));
        self._backreferences.borrow_mut().clear();
        self._capturing_groups.borrow_mut().clear();
    }

    fn on_pattern_leave(&self, start: usize, end: usize) {
        self._arena
            .node_mut(self._node.borrow().unwrap())
            .thrush(|mut node| {
                node.set_end(end);
                node.set_raw(self.source[start..end].to_owned());
            });

        for &reference in &*self._backreferences.borrow() {
            let ref_ = self._arena.node(reference).as_backreference().ref_.clone();
            let group = match ref_ {
                CapturingGroupKey::Index(ref_) => self._capturing_groups.borrow()[ref_ - 1],
                CapturingGroupKey::Name(ref_) => *self
                    ._capturing_groups
                    .borrow()
                    .iter()
                    .find(|&&g| {
                        self._arena.node(g).as_capturing_group().name.as_ref() == Some(&ref_)
                    })
                    .unwrap(),
            };
            self._arena
                .node_mut(reference)
                .as_backreference_mut()
                .resolved = group;
            self._arena
                .node_mut(group)
                .as_capturing_group_mut()
                .references
                .push(reference);
        }
    }

    fn on_alternative_enter(&self, start: usize, index: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::Assertion(_) | Node::CapturingGroup(_) | Node::Group(_) | Node::Pattern(_)
        ));

        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_alternative(
            Some(parent),
            start,
            start,
            Default::default(),
            Default::default(),
        )));
        let _node = self._node.borrow().unwrap();
        match &mut *self._arena.node_mut(parent) {
            Node::Assertion(parent) => parent.alternatives.as_mut().unwrap().push(_node),
            Node::CapturingGroup(parent) => parent.alternatives.push(_node),
            Node::Group(parent) => parent.alternatives.push(_node),
            Node::Pattern(parent) => parent.alternatives.push(_node),
            _ => unreachable!(),
        }
    }

    fn on_alternative_leave(&self, start: usize, end: usize, index: usize) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(node), Node::Alternative(_)));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = self._arena.node(node).maybe_parent();
    }

    fn on_group_enter(&self, start: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_group(
            Some(parent),
            start,
            start,
            Default::default(),
            Default::default(),
        )));
        let _node = self._node.borrow().unwrap();
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(_node);
    }

    fn on_group_leave(&self, start: usize, end: usize) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(node),
            Node::Group(_) | Node::Alternative(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = self._arena.node(node).maybe_parent();
    }

    fn on_capturing_group_enter(&self, start: usize, name: Option<&str>) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_capturing_group(
            Some(parent),
            start,
            start,
            Default::default(),
            name.map(ToOwned::to_owned),
            Default::default(),
            Default::default(),
        )));
    }

    fn on_capturing_group_leave(&self, start: usize, end: usize, _name: Option<&str>) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(node),
            Node::CapturingGroup(_) | Node::Alternative(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = self._arena.node(node).maybe_parent();
    }

    fn on_quantifier(&self, start: usize, end: usize, min: usize, max: usize, greedy: bool) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        let element = self
            ._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .pop()
            .unwrap();
        assert!(
            !matches!(
                &*self._arena.node(element),
                Node::Quantifier(_)
            ) && !matches!(
                &*self._arena.node(element),
                Node::Assertion(element) if element.kind == AssertionKind::Lookahead,
            )
        );

        let node = Node::new_quantifier(
            Some(parent),
            self._arena.node(element).start(),
            end,
            self.source[self._arena.node(element).start()..end].to_owned(),
            min,
            max,
            greedy,
            element,
        );
        let node = self._arena.alloc_node(node);
        self._arena.node_mut(parent).as_alternative_mut().elements.push(node);
        self._arena.node_mut(element).set_parent(Some(node));
    }

    fn on_literal_enter(&self, start: usize) {}

    fn on_literal_leave(&self, start: usize, end: usize) {}

    fn on_disjunction_enter(&self, start: usize) {}

    fn on_disjunction_leave(&self, start: usize, end: usize) {}

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

    fn on_backreference(&self, start: usize, TODO: usize, a: CapturingGroupKey) {}

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
