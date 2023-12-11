use std::{cell::RefCell, collections::HashMap, rc::Rc};

use id_arena::Id;
use serde::Deserialize;
use squalid::EverythingExt;

use crate::{
    arena::AllArenas,
    ast::{Flags, Node, NodeBase, NodeInterface},
    ecma_versions::{EcmaVersion, LATEST_ECMA_VERSION},
    unicode::HYPHEN_MINUS,
    validator::{self, AssertionKind, CapturingGroupKey, CharacterKind, RegExpFlags},
    CodePoint, RegExpValidator, Result,
};

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
}

fn is_class_set_operand(node: &Node) -> bool {
    matches!(
        node,
        Node::Character(_)
            | Node::CharacterSet(_)
            | Node::CharacterClass(_)
            | Node::ExpressionCharacterClass(_)
            | Node::ClassStringDisjunction(_)
    )
}

struct RegExpParserState<'a> {
    _arena: &'a AllArenas,
    strict: bool,
    ecma_version: EcmaVersion,
    _node: RefCell<Option<Id<Node> /*AppendableNode*/>>,
    _expression_buffer_map: RefCell<HashMap<Id<Node>, Id<Node>>>,
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
                .resolved = Some(group);
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
            !matches!(&*self._arena.node(element), Node::Quantifier(_))
                && !matches!(
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
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
        self._arena.node_mut(element).set_parent(Some(node));
    }

    fn on_lookaround_assertion_enter(&self, start: usize, kind: AssertionKind, negate: bool) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_assertion(
            Some(parent),
            start,
            start,
            Default::default(),
            kind,
            Some(negate),
            Some(Default::default()),
        )));
        let node = self._node.borrow().unwrap();
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
    }

    fn on_lookaround_assertion_leave(
        &self,
        start: usize,
        end: usize,
        _kind: AssertionKind,
        _negate: bool,
    ) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(node),
            Node::Assertion(_) | Node::Alternative(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = self._arena.node(node).maybe_parent();
    }

    fn on_edge_assertion(&self, start: usize, end: usize, kind: AssertionKind) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        let node = self._arena.alloc_node(Node::new_assertion(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            kind,
            None,
            None,
        ));
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
    }

    fn on_word_boundary_assertion(
        &self,
        start: usize,
        end: usize,
        kind: AssertionKind,
        negate: bool,
    ) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        let node = self._arena.alloc_node(Node::new_assertion(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            kind,
            Some(negate),
            None,
        ));
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
    }

    fn on_any_character_set(&self, start: usize, end: usize, kind: CharacterKind) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        let node = self._arena.alloc_node(Node::new_character_set(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            kind,
            None,
            None,
            None,
            None,
        ));
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
    }

    fn on_escape_character_set(&self, start: usize, end: usize, kind: CharacterKind, negate: bool) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::Alternative(_) | Node::CharacterClass(_)
        ));

        let node = self._arena.alloc_node(Node::new_character_set(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            kind,
            None,
            None,
            None,
            Some(negate),
        ));
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
    }

    fn on_unicode_property_character_set(
        &self,
        start: usize,
        end: usize,
        kind: CharacterKind,
        key: &str,
        value: Option<&str>,
        negate: bool,
        strings: bool,
    ) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::Alternative(_) | Node::CharacterClass(_)
        ));

        if strings {
            assert!(
                !(matches!(
                    &*self._arena.node(parent),
                    Node::CharacterClass(parent) if !parent.unicode_sets
                ) || negate
                    || value.is_some())
            );
        }

        let node = self._arena.alloc_node(Node::new_character_set(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            kind,
            Some(strings),
            Some(key.to_owned()),
            value.map(ToOwned::to_owned),
            Some(negate),
        ));
        match &mut *self._arena.node_mut(parent) {
            Node::Alternative(parent) => parent.elements.push(node),
            Node::CharacterClass(parent) => parent.elements.push(node),
            _ => unreachable!(),
        }
    }

    fn on_character(&self, start: usize, end: usize, value: CodePoint) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::Alternative(_) | Node::CharacterClass(_) | Node::StringAlternative(_)
        ));

        let node = self._arena.alloc_node(Node::new_character(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            value,
        ));
        match &mut *self._arena.node_mut(parent) {
            Node::Alternative(parent) => parent.elements.push(node),
            Node::CharacterClass(parent) => parent.elements.push(node),
            Node::StringAlternative(parent) => parent.elements.push(node),
            _ => unreachable!(),
        }
    }

    fn on_backreference(&self, start: usize, end: usize, ref_: &CapturingGroupKey) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(parent), Node::Alternative(_)));

        let node = self._arena.alloc_node(Node::new_backreference(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            ref_.clone(),
            Default::default(),
        ));
        self._arena
            .node_mut(parent)
            .as_alternative_mut()
            .elements
            .push(node);
        self._backreferences.borrow_mut().push(node);
    }

    fn on_character_class_enter(&self, start: usize, negate: bool, unicode_sets: bool) {
        let parent = self._node.borrow().unwrap();
        assert!(
            matches!(&*self._arena.node(parent), Node::Alternative(_))
                || matches!(
                    &*self._arena.node(parent),
                    Node::CharacterClass(node) if node.unicode_sets && unicode_sets
                )
        );
        let node = self._arena.alloc_node(Node::new_character_class(
            Some(parent),
            start,
            start,
            Default::default(),
            unicode_sets,
            negate,
            Default::default(),
        ));
        *self._node.borrow_mut() = Some(node);
        match &mut *self._arena.node_mut(parent) {
            Node::Alternative(parent) => parent.elements.push(node),
            Node::CharacterClass(parent) => parent.elements.push(node),
            _ => unreachable!(),
        }
    }

    fn on_character_class_leave(&self, start: usize, end: usize, _negate: bool) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(&*self._arena.node(node), Node::CharacterClass(_)));
        let parent = self._arena.node(node).maybe_parent().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::Alternative(_) | Node::CharacterClass(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = Some(parent);

        let Some(expression) = self._expression_buffer_map.borrow().get(&node).copied() else {
            return;
        };
        assert!(self
            ._arena
            .node(node)
            .as_character_class()
            .elements
            .is_empty());
        self._expression_buffer_map.borrow_mut().remove(&node);

        let new_node = self._arena.alloc_node(Node::new_expression_character_class(
            Some(parent),
            self._arena.node(node).start(),
            self._arena.node(node).end(),
            self._arena.node(node).raw().to_owned(),
            self._arena.node(node).as_character_class().negate,
            expression,
        ));
        self._arena.node_mut(expression).set_parent(Some(new_node));
        assert!(
            Some(node)
                == match &mut *self._arena.node_mut(parent) {
                    Node::Alternative(parent) => parent.elements.pop(),
                    Node::CharacterClass(parent) => parent.elements.pop(),
                    _ => unreachable!(),
                }
        );
        match &mut *self._arena.node_mut(parent) {
            Node::Alternative(parent) => parent.elements.push(new_node),
            Node::CharacterClass(parent) => parent.elements.push(new_node),
            _ => unreachable!(),
        }
    }

    fn on_character_class_range(&self, start: usize, end: usize, _min: CodePoint, _max: CodePoint) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::CharacterClass(_)
        ));
        let max = self
            ._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .pop()
            .unwrap();
        assert!(matches!(&*self._arena.node(max), Node::Character(_)));
        if !self._arena.node(parent).as_character_class().unicode_sets {
            let hyphen = self
                ._arena
                .node_mut(parent)
                .as_character_class_mut()
                .elements
                .pop()
                .unwrap();
            assert!(matches!(
                &*self._arena.node(hyphen),
                Node::Character(hyphen) if hyphen.value == HYPHEN_MINUS
            ));
        }
        let min = self
            ._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .pop()
            .unwrap();
        assert!(matches!(&*self._arena.node(min), Node::Character(_)));

        let node = self._arena.alloc_node(Node::new_character_class_range(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            min,
            max,
        ));
        self._arena.node_mut(min).set_parent(Some(node));
        self._arena.node_mut(max).set_parent(Some(node));
        self._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .push(node);
    }

    fn on_class_intersection(&self, start: usize, end: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::CharacterClass(parent) if parent.unicode_sets
        ));
        let right = self
            ._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .pop()
            .unwrap();
        let left = self
            ._expression_buffer_map
            .borrow()
            .get(&parent)
            .copied()
            .unwrap_or_else(|| {
                self._arena
                    .node_mut(parent)
                    .as_character_class_mut()
                    .elements
                    .pop()
                    .unwrap()
            });
        assert!(!matches!(
            &*self._arena.node(left),
            Node::ClassSubtraction(_)
        ),);
        assert!(
            !(!matches!(&*self._arena.node(left), Node::ClassIntersection(_))
                && !is_class_set_operand(&self._arena.node(left)))
        );
        assert!(is_class_set_operand(&self._arena.node(right)));
        let node = self._arena.alloc_node(Node::new_class_intersection(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            left,
            right,
        ));
        self._arena.node_mut(left).set_parent(Some(node));
        self._arena.node_mut(right).set_parent(Some(node));
        self._expression_buffer_map
            .borrow_mut()
            .insert(parent, node);
    }

    fn on_class_subtraction(&self, start: usize, end: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::CharacterClass(parent) if parent.unicode_sets
        ));

        let right = self
            ._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .pop()
            .unwrap();
        let left = self
            ._expression_buffer_map
            .borrow()
            .get(&parent)
            .copied()
            .unwrap_or_else(|| {
                self._arena
                    .node_mut(parent)
                    .as_character_class_mut()
                    .elements
                    .pop()
                    .unwrap()
            });
        assert!(!matches!(
            &*self._arena.node(left),
            Node::ClassIntersection(_)
        ),);
        assert!(
            !(!matches!(&*self._arena.node(left), Node::ClassSubtraction(_))
                && !is_class_set_operand(&self._arena.node(left)))
        );
        assert!(is_class_set_operand(&self._arena.node(right)));
        let node = self._arena.alloc_node(Node::new_class_subtraction(
            Some(parent),
            start,
            end,
            self.source[start..end].to_owned(),
            left,
            right,
        ));
        self._arena.node_mut(left).set_parent(Some(node));
        self._arena.node_mut(right).set_parent(Some(node));
        self._expression_buffer_map
            .borrow_mut()
            .insert(parent, node);
    }

    fn on_class_string_disjunction_enter(&self, start: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::CharacterClass(parent) if parent.unicode_sets
        ));

        *self._node.borrow_mut() =
            Some(self._arena.alloc_node(Node::new_class_string_disjunction(
                Some(parent),
                start,
                start,
                Default::default(),
                Default::default(),
            )));
        let node = self._node.borrow().unwrap();
        self._arena
            .node_mut(parent)
            .as_character_class_mut()
            .elements
            .push(node);
    }

    fn on_class_string_disjunction_leave(&self, start: usize, end: usize) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(node),
            Node::ClassStringDisjunction(_)
        ));
        let parent = self._arena.node(node).maybe_parent().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::CharacterClass(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = Some(parent);
    }

    fn on_string_alternative_enter(&self, start: usize, _index: usize) {
        let parent = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(parent),
            Node::ClassStringDisjunction(_)
        ));

        *self._node.borrow_mut() = Some(self._arena.alloc_node(Node::new_string_alternative(
            Some(parent),
            start,
            start,
            Default::default(),
            Default::default(),
        )));
        let node = self._node.borrow().unwrap();
        self._arena
            .node_mut(parent)
            .as_class_string_disjunction_mut()
            .alternatives
            .push(node);
    }

    fn on_string_alternative_leave(&self, start: usize, end: usize, _index: usize) {
        let node = self._node.borrow().unwrap();
        assert!(matches!(
            &*self._arena.node(node),
            Node::StringAlternative(_)
        ));

        self._arena.node_mut(node).thrush(|mut node| {
            node.set_end(end);
            node.set_raw(self.source[start..end].to_owned());
        });
        *self._node.borrow_mut() = self._arena.node(node).maybe_parent();
    }
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
