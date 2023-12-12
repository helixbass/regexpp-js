use std::{collections::HashMap, mem, ops};

use id_arena::Id;
use pathdiff::diff_paths;
use serde::{Deserialize, Deserializer};
use serde_bytes::ByteBuf;
use wtf8::Wtf8;

use crate::{
    arena::AllArenas,
    validator::{AssertionKind, CapturingGroupKey, CharacterKind},
    CodePoint, Wtf16,
};

#[derive(Clone)]
pub enum Node {
    Alternative(Alternative),
    CapturingGroup(CapturingGroup),
    CharacterClass(CharacterClass),
    CharacterClassRange(CharacterClassRange),
    ClassIntersection(ClassIntersection),
    ClassStringDisjunction(ClassStringDisjunction),
    ClassSubtraction(ClassSubtraction),
    ExpressionCharacterClass(ExpressionCharacterClass),
    Group(Group),
    Assertion(Assertion),
    Pattern(Pattern),
    Quantifier(Quantifier),
    RegExpLiteral(RegExpLiteral),
    StringAlternative(StringAlternative),
    Backreference(Backreference),
    Character(Character),
    CharacterSet(CharacterSet),
    Flags(Flags),
}

impl Node {
    #[allow(clippy::too_many_arguments)]
    pub fn new_flags(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        dot_all: bool,
        global: bool,
        has_indices: bool,
        ignore_case: bool,
        multiline: bool,
        sticky: bool,
        unicode: bool,
        unicode_sets: bool,
    ) -> Self {
        Self::Flags(Flags {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            dot_all,
            global,
            has_indices,
            ignore_case,
            multiline,
            sticky,
            unicode,
            unicode_sets,
        })
    }

    pub fn new_pattern(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        alternatives: Vec<Id<Node>>,
    ) -> Self {
        Self::Pattern(Pattern {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            alternatives,
        })
    }

    pub fn new_alternative(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        elements: Vec<Id<Node>>,
    ) -> Self {
        Self::Alternative(Alternative {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            elements,
        })
    }

    pub fn new_group(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        alternatives: Vec<Id<Node>>,
    ) -> Self {
        Self::Group(Group {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            alternatives,
        })
    }

    pub fn new_capturing_group(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        name: Option<Wtf16>,
        alternatives: Vec<Id<Node>>,
        references: Vec<Id<Node>>,
    ) -> Self {
        Self::CapturingGroup(CapturingGroup {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            name,
            alternatives,
            references,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_quantifier(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        min: u32,
        max: u32,
        greedy: bool,
        element: Id<Node>,
    ) -> Self {
        Self::Quantifier(Quantifier {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            min,
            max,
            greedy,
            element,
        })
    }

    pub fn new_assertion(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        kind: AssertionKind,
        negate: Option<bool>,
        alternatives: Option<Vec<Id<Node>>>,
    ) -> Self {
        Self::Assertion(Assertion {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            kind,
            negate,
            alternatives,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_character_set(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        kind: CharacterKind,
        strings: Option<bool>,
        key: Option<Wtf16>,
        value: Option<Wtf16>,
        negate: Option<bool>,
    ) -> Self {
        Self::CharacterSet(CharacterSet {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            kind,
            strings,
            key,
            value,
            negate,
        })
    }

    pub fn new_character(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        value: CodePoint,
    ) -> Self {
        Self::Character(Character {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            value,
        })
    }

    pub fn new_backreference(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        ref_: CapturingGroupKey,
        resolved: Option<Id<Node>>,
    ) -> Self {
        Self::Backreference(Backreference {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            ref_,
            resolved,
        })
    }

    pub fn new_character_class(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        unicode_sets: bool,
        negate: bool,
        elements: Vec<Id<Node> /*CharacterClassElement*/>,
    ) -> Self {
        Self::CharacterClass(CharacterClass {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            unicode_sets,
            negate,
            elements,
        })
    }

    pub fn new_expression_character_class(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        negate: bool,
        expression: Id<Node>,
    ) -> Self {
        Self::ExpressionCharacterClass(ExpressionCharacterClass {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            negate,
            expression,
        })
    }

    pub fn new_character_class_range(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        min: Id<Node>,
        max: Id<Node>,
    ) -> Self {
        Self::CharacterClassRange(CharacterClassRange {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            min,
            max,
        })
    }

    pub fn new_class_intersection(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        left: Id<Node>,
        right: Id<Node>,
    ) -> Self {
        Self::ClassIntersection(ClassIntersection {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            left,
            right,
        })
    }

    pub fn new_class_subtraction(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        left: Id<Node>,
        right: Id<Node>,
    ) -> Self {
        Self::ClassSubtraction(ClassSubtraction {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            left,
            right,
        })
    }

    pub fn new_class_string_disjunction(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        alternatives: Vec<Id<Node>>,
    ) -> Self {
        Self::ClassStringDisjunction(ClassStringDisjunction {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            alternatives,
        })
    }

    pub fn new_string_alternative(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        elements: Vec<Id<Node>>,
    ) -> Self {
        Self::StringAlternative(StringAlternative {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            elements,
        })
    }

    pub fn new_reg_exp_literal(
        parent: Option<Id<Node>>,
        start: usize,
        end: usize,
        raw: Wtf16,
        pattern: Id<Node>,
        flags: Id<Node>,
    ) -> Self {
        Self::RegExpLiteral(RegExpLiteral {
            _base: NodeBase {
                _arena_id: Default::default(),
                parent,
                start,
                end,
                raw,
            },
            pattern,
            flags,
        })
    }

    pub fn as_backreference(&self) -> &Backreference {
        match self {
            Self::Backreference(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_backreference_mut(&mut self) -> &mut Backreference {
        match self {
            Self::Backreference(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_capturing_group(&self) -> &CapturingGroup {
        match self {
            Self::CapturingGroup(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_capturing_group_mut(&mut self) -> &mut CapturingGroup {
        match self {
            Self::CapturingGroup(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_alternative_mut(&mut self) -> &mut Alternative {
        match self {
            Self::Alternative(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_character_class(&self) -> &CharacterClass {
        match self {
            Self::CharacterClass(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_character_class_mut(&mut self) -> &mut CharacterClass {
        match self {
            Self::CharacterClass(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn as_class_string_disjunction_mut(&mut self) -> &mut ClassStringDisjunction {
        match self {
            Self::ClassStringDisjunction(value) => value,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum NodeUnresolved {
    Alternative(Box<AlternativeUnresolved>),
    CapturingGroup(Box<CapturingGroupUnresolved>),
    CharacterClass(Box<CharacterClassUnresolved>),
    CharacterClassRange(Box<CharacterClassRangeUnresolved>),
    ClassIntersection(Box<ClassIntersectionUnresolved>),
    ClassStringDisjunction(Box<ClassStringDisjunctionUnresolved>),
    ClassSubtraction(Box<ClassSubtractionUnresolved>),
    ExpressionCharacterClass(Box<ExpressionCharacterClassUnresolved>),
    Group(Box<GroupUnresolved>),
    Assertion(Box<AssertionUnresolved>),
    Pattern(Box<PatternUnresolved>),
    Quantifier(Box<QuantifierUnresolved>),
    RegExpLiteral(Box<RegExpLiteralUnresolved>),
    StringAlternative(Box<StringAlternativeUnresolved>),
    Backreference(Box<BackreferenceUnresolved>),
    Character(Box<CharacterUnresolved>),
    CharacterSet(Box<CharacterSetUnresolved>),
    Flags(Box<FlagsUnresolved>),
}

pub trait NodeInterface {
    fn set_arena_id(&mut self, id: Id<Node>);
    fn maybe_parent(&self) -> Option<Id<Node>>;
    fn set_parent(&mut self, parent: Option<Id<Node>>);
    fn parent(&self) -> Id<Node>;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn set_end(&mut self, end: usize);
    fn raw(&self) -> &[u16];
    fn set_raw(&mut self, raw: Wtf16);
}

impl NodeInterface for Node {
    fn set_arena_id(&mut self, id: Id<Node>) {
        match self {
            Node::Alternative(node) => node._base.set_arena_id(id),
            Node::CapturingGroup(node) => node._base.set_arena_id(id),
            Node::CharacterClass(node) => node._base.set_arena_id(id),
            Node::CharacterClassRange(node) => node._base.set_arena_id(id),
            Node::ClassIntersection(node) => node._base.set_arena_id(id),
            Node::ClassStringDisjunction(node) => node._base.set_arena_id(id),
            Node::ClassSubtraction(node) => node._base.set_arena_id(id),
            Node::ExpressionCharacterClass(node) => node._base.set_arena_id(id),
            Node::Group(node) => node._base.set_arena_id(id),
            Node::Assertion(node) => node._base.set_arena_id(id),
            Node::Pattern(node) => node._base.set_arena_id(id),
            Node::Quantifier(node) => node._base.set_arena_id(id),
            Node::RegExpLiteral(node) => node._base.set_arena_id(id),
            Node::StringAlternative(node) => node._base.set_arena_id(id),
            Node::Backreference(node) => node._base.set_arena_id(id),
            Node::Character(node) => node._base.set_arena_id(id),
            Node::CharacterSet(node) => node._base.set_arena_id(id),
            Node::Flags(node) => node._base.set_arena_id(id),
        }
    }

    fn maybe_parent(&self) -> Option<Id<Node>> {
        match self {
            Node::Alternative(node) => node._base.maybe_parent(),
            Node::CapturingGroup(node) => node._base.maybe_parent(),
            Node::CharacterClass(node) => node._base.maybe_parent(),
            Node::CharacterClassRange(node) => node._base.maybe_parent(),
            Node::ClassIntersection(node) => node._base.maybe_parent(),
            Node::ClassStringDisjunction(node) => node._base.maybe_parent(),
            Node::ClassSubtraction(node) => node._base.maybe_parent(),
            Node::ExpressionCharacterClass(node) => node._base.maybe_parent(),
            Node::Group(node) => node._base.maybe_parent(),
            Node::Assertion(node) => node._base.maybe_parent(),
            Node::Pattern(node) => node._base.maybe_parent(),
            Node::Quantifier(node) => node._base.maybe_parent(),
            Node::RegExpLiteral(node) => node._base.maybe_parent(),
            Node::StringAlternative(node) => node._base.maybe_parent(),
            Node::Backreference(node) => node._base.maybe_parent(),
            Node::Character(node) => node._base.maybe_parent(),
            Node::CharacterSet(node) => node._base.maybe_parent(),
            Node::Flags(node) => node._base.maybe_parent(),
        }
    }

    fn set_parent(&mut self, parent: Option<Id<Node>>) {
        match self {
            Node::Alternative(node) => node._base.set_parent(parent),
            Node::CapturingGroup(node) => node._base.set_parent(parent),
            Node::CharacterClass(node) => node._base.set_parent(parent),
            Node::CharacterClassRange(node) => node._base.set_parent(parent),
            Node::ClassIntersection(node) => node._base.set_parent(parent),
            Node::ClassStringDisjunction(node) => node._base.set_parent(parent),
            Node::ClassSubtraction(node) => node._base.set_parent(parent),
            Node::ExpressionCharacterClass(node) => node._base.set_parent(parent),
            Node::Group(node) => node._base.set_parent(parent),
            Node::Assertion(node) => node._base.set_parent(parent),
            Node::Pattern(node) => node._base.set_parent(parent),
            Node::Quantifier(node) => node._base.set_parent(parent),
            Node::RegExpLiteral(node) => node._base.set_parent(parent),
            Node::StringAlternative(node) => node._base.set_parent(parent),
            Node::Backreference(node) => node._base.set_parent(parent),
            Node::Character(node) => node._base.set_parent(parent),
            Node::CharacterSet(node) => node._base.set_parent(parent),
            Node::Flags(node) => node._base.set_parent(parent),
        }
    }

    fn parent(&self) -> Id<Node> {
        match self {
            Node::Alternative(node) => node._base.parent(),
            Node::CapturingGroup(node) => node._base.parent(),
            Node::CharacterClass(node) => node._base.parent(),
            Node::CharacterClassRange(node) => node._base.parent(),
            Node::ClassIntersection(node) => node._base.parent(),
            Node::ClassStringDisjunction(node) => node._base.parent(),
            Node::ClassSubtraction(node) => node._base.parent(),
            Node::ExpressionCharacterClass(node) => node._base.parent(),
            Node::Group(node) => node._base.parent(),
            Node::Assertion(node) => node._base.parent(),
            Node::Pattern(node) => node._base.parent(),
            Node::Quantifier(node) => node._base.parent(),
            Node::RegExpLiteral(node) => node._base.parent(),
            Node::StringAlternative(node) => node._base.parent(),
            Node::Backreference(node) => node._base.parent(),
            Node::Character(node) => node._base.parent(),
            Node::CharacterSet(node) => node._base.parent(),
            Node::Flags(node) => node._base.parent(),
        }
    }

    fn start(&self) -> usize {
        match self {
            Node::Alternative(node) => node._base.start(),
            Node::CapturingGroup(node) => node._base.start(),
            Node::CharacterClass(node) => node._base.start(),
            Node::CharacterClassRange(node) => node._base.start(),
            Node::ClassIntersection(node) => node._base.start(),
            Node::ClassStringDisjunction(node) => node._base.start(),
            Node::ClassSubtraction(node) => node._base.start(),
            Node::ExpressionCharacterClass(node) => node._base.start(),
            Node::Group(node) => node._base.start(),
            Node::Assertion(node) => node._base.start(),
            Node::Pattern(node) => node._base.start(),
            Node::Quantifier(node) => node._base.start(),
            Node::RegExpLiteral(node) => node._base.start(),
            Node::StringAlternative(node) => node._base.start(),
            Node::Backreference(node) => node._base.start(),
            Node::Character(node) => node._base.start(),
            Node::CharacterSet(node) => node._base.start(),
            Node::Flags(node) => node._base.start(),
        }
    }

    fn end(&self) -> usize {
        match self {
            Node::Alternative(node) => node._base.end(),
            Node::CapturingGroup(node) => node._base.end(),
            Node::CharacterClass(node) => node._base.end(),
            Node::CharacterClassRange(node) => node._base.end(),
            Node::ClassIntersection(node) => node._base.end(),
            Node::ClassStringDisjunction(node) => node._base.end(),
            Node::ClassSubtraction(node) => node._base.end(),
            Node::ExpressionCharacterClass(node) => node._base.end(),
            Node::Group(node) => node._base.end(),
            Node::Assertion(node) => node._base.end(),
            Node::Pattern(node) => node._base.end(),
            Node::Quantifier(node) => node._base.end(),
            Node::RegExpLiteral(node) => node._base.end(),
            Node::StringAlternative(node) => node._base.end(),
            Node::Backreference(node) => node._base.end(),
            Node::Character(node) => node._base.end(),
            Node::CharacterSet(node) => node._base.end(),
            Node::Flags(node) => node._base.end(),
        }
    }

    fn set_end(&mut self, end: usize) {
        match self {
            Node::Alternative(node) => node._base.set_end(end),
            Node::CapturingGroup(node) => node._base.set_end(end),
            Node::CharacterClass(node) => node._base.set_end(end),
            Node::CharacterClassRange(node) => node._base.set_end(end),
            Node::ClassIntersection(node) => node._base.set_end(end),
            Node::ClassStringDisjunction(node) => node._base.set_end(end),
            Node::ClassSubtraction(node) => node._base.set_end(end),
            Node::ExpressionCharacterClass(node) => node._base.set_end(end),
            Node::Group(node) => node._base.set_end(end),
            Node::Assertion(node) => node._base.set_end(end),
            Node::Pattern(node) => node._base.set_end(end),
            Node::Quantifier(node) => node._base.set_end(end),
            Node::RegExpLiteral(node) => node._base.set_end(end),
            Node::StringAlternative(node) => node._base.set_end(end),
            Node::Backreference(node) => node._base.set_end(end),
            Node::Character(node) => node._base.set_end(end),
            Node::CharacterSet(node) => node._base.set_end(end),
            Node::Flags(node) => node._base.set_end(end),
        }
    }

    fn raw(&self) -> &[u16] {
        match self {
            Node::Alternative(node) => node._base.raw(),
            Node::CapturingGroup(node) => node._base.raw(),
            Node::CharacterClass(node) => node._base.raw(),
            Node::CharacterClassRange(node) => node._base.raw(),
            Node::ClassIntersection(node) => node._base.raw(),
            Node::ClassStringDisjunction(node) => node._base.raw(),
            Node::ClassSubtraction(node) => node._base.raw(),
            Node::ExpressionCharacterClass(node) => node._base.raw(),
            Node::Group(node) => node._base.raw(),
            Node::Assertion(node) => node._base.raw(),
            Node::Pattern(node) => node._base.raw(),
            Node::Quantifier(node) => node._base.raw(),
            Node::RegExpLiteral(node) => node._base.raw(),
            Node::StringAlternative(node) => node._base.raw(),
            Node::Backreference(node) => node._base.raw(),
            Node::Character(node) => node._base.raw(),
            Node::CharacterSet(node) => node._base.raw(),
            Node::Flags(node) => node._base.raw(),
        }
    }

    fn set_raw(&mut self, raw: Wtf16) {
        match self {
            Node::Alternative(node) => node._base.set_raw(raw),
            Node::CapturingGroup(node) => node._base.set_raw(raw),
            Node::CharacterClass(node) => node._base.set_raw(raw),
            Node::CharacterClassRange(node) => node._base.set_raw(raw),
            Node::ClassIntersection(node) => node._base.set_raw(raw),
            Node::ClassStringDisjunction(node) => node._base.set_raw(raw),
            Node::ClassSubtraction(node) => node._base.set_raw(raw),
            Node::ExpressionCharacterClass(node) => node._base.set_raw(raw),
            Node::Group(node) => node._base.set_raw(raw),
            Node::Assertion(node) => node._base.set_raw(raw),
            Node::Pattern(node) => node._base.set_raw(raw),
            Node::Quantifier(node) => node._base.set_raw(raw),
            Node::RegExpLiteral(node) => node._base.set_raw(raw),
            Node::StringAlternative(node) => node._base.set_raw(raw),
            Node::Backreference(node) => node._base.set_raw(raw),
            Node::Character(node) => node._base.set_raw(raw),
            Node::CharacterSet(node) => node._base.set_raw(raw),
            Node::Flags(node) => node._base.set_raw(raw),
        }
    }
}

fn resolve_location_vec(
    arena: &AllArenas,
    nodes: &[Id<Node>],
    vec_property_name: &str,
    path: &mut Vec<String>,
    path_map: &mut HashMap<Id<Node>, String>,
) {
    path.push(vec_property_name.to_owned());
    for (index, &node) in nodes.iter().enumerate() {
        path.push(index.to_string());
        resolve_location(arena, node, path, path_map);
        path.pop();
    }
    path.pop();
}

pub fn resolve_location(
    arena: &AllArenas,
    node: Id<Node>,
    path: &mut Vec<String>,
    path_map: &mut HashMap<Id<Node>, String>,
) {
    path_map.insert(node, format!("/{}", path.join("/")));
    match &*arena.node(node) {
        Node::Alternative(node) => {
            resolve_location_vec(arena, &node.elements, "elements", path, path_map);
        }
        Node::CapturingGroup(node) => {
            resolve_location_vec(arena, &node.alternatives, "alternatives", path, path_map);
        }
        Node::CharacterClass(node) => {
            resolve_location_vec(arena, &node.elements, "elements", path, path_map);
        }
        Node::CharacterClassRange(node) => {
            path.push("min".to_owned());
            resolve_location(arena, node.min, path, path_map);
            path.pop();

            path.push("max".to_owned());
            resolve_location(arena, node.max, path, path_map);
            path.pop();
        }
        Node::ClassIntersection(node) => {
            path.push("left".to_owned());
            resolve_location(arena, node.left, path, path_map);
            path.pop();

            path.push("right".to_owned());
            resolve_location(arena, node.right, path, path_map);
            path.pop();
        }
        Node::ClassStringDisjunction(node) => {
            resolve_location_vec(arena, &node.alternatives, "alternatives", path, path_map);
        }
        Node::ClassSubtraction(node) => {
            path.push("left".to_owned());
            resolve_location(arena, node.left, path, path_map);
            path.pop();

            path.push("right".to_owned());
            resolve_location(arena, node.right, path, path_map);
            path.pop();
        }
        Node::ExpressionCharacterClass(node) => {
            path.push("expression".to_owned());
            resolve_location(arena, node.expression, path, path_map);
            path.pop();
        }
        Node::Group(node) => {
            resolve_location_vec(arena, &node.alternatives, "alternatives", path, path_map);
        }
        Node::Assertion(node) => {
            if let Some(alternatives) = node.alternatives.as_ref() {
                resolve_location_vec(arena, alternatives, "alternatives", path, path_map);
            }
        }
        Node::Pattern(node) => {
            resolve_location_vec(arena, &node.alternatives, "alternatives", path, path_map);
        }
        Node::Quantifier(node) => {
            path.push("element".to_owned());
            resolve_location(arena, node.element, path, path_map);
            path.pop();
        }
        Node::RegExpLiteral(node) => {
            path.push("pattern".to_owned());
            resolve_location(arena, node.pattern, path, path_map);
            path.pop();

            path.push("flags".to_owned());
            resolve_location(arena, node.flags, path, path_map);
            path.pop();
        }
        Node::StringAlternative(node) => {
            resolve_location_vec(arena, &node.elements, "elements", path, path_map);
        }
        _ => (),
    }
}

#[derive(Clone)]
pub struct NodeBase {
    _arena_id: Option<Id<Node>>,
    // type: Node["type"],
    parent: Option<Id<Node>>,
    start: usize,
    end: usize,
    raw: Wtf16,
}

impl NodeInterface for NodeBase {
    fn set_arena_id(&mut self, id: Id<Node>) {
        self._arena_id = Some(id);
    }

    fn maybe_parent(&self) -> Option<Id<Node>> {
        self.parent
    }

    fn parent(&self) -> Id<Node> {
        self.parent.unwrap()
    }

    fn set_parent(&mut self, parent: Option<Id<Node>>) {
        self.parent = parent;
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    fn raw(&self) -> &[u16] {
        &self.raw
    }

    fn set_raw(&mut self, raw: Wtf16) {
        self.raw = raw;
    }
}

fn get_relative_path(
    from: Id<Node>,
    to: Id<Node>,
    path_map: &HashMap<Id<Node>, String>,
) -> String {
    let from_path = &path_map[&from];
    let to_path = &path_map[&to];
    let relative = diff_paths(to_path, from_path).unwrap();
    let relative = relative.to_str().unwrap();
    format!("♻️{}", relative.strip_suffix('/').unwrap_or(relative),)
}

pub fn to_node_unresolved(
    id: Id<Node>,
    arena: &AllArenas,
    path_map: &HashMap<Id<Node>, String>,
) -> NodeUnresolved {
    match &*arena.node(id) {
        Node::Alternative(node) => NodeUnresolved::Alternative(Box::new(AlternativeUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            elements: node
                .elements
                .iter()
                .map(|&node| to_node_unresolved(node, arena, path_map))
                .collect(),
        })),
        Node::CapturingGroup(node) => {
            NodeUnresolved::CapturingGroup(Box::new(CapturingGroupUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                name: node.name.clone(),
                alternatives: node
                    .alternatives
                    .iter()
                    .map(|&node| to_node_unresolved(node, arena, path_map))
                    .collect(),
                references: node
                    .references
                    .iter()
                    .map(|&reference| {
                        get_relative_path(node._base._arena_id.unwrap(), reference, path_map)
                    })
                    .collect(),
            }))
        }
        Node::CharacterClass(node) => {
            NodeUnresolved::CharacterClass(Box::new(CharacterClassUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                unicode_sets: node.unicode_sets,
                negate: node.negate,
                elements: node
                    .elements
                    .iter()
                    .map(|&node| to_node_unresolved(node, arena, path_map))
                    .collect(),
            }))
        }
        Node::CharacterClassRange(node) => {
            NodeUnresolved::CharacterClassRange(Box::new(CharacterClassRangeUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                min: to_node_unresolved(node.min, arena, path_map),
                max: to_node_unresolved(node.max, arena, path_map),
            }))
        }
        Node::ClassIntersection(node) => {
            NodeUnresolved::ClassIntersection(Box::new(ClassIntersectionUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                left: to_node_unresolved(node.left, arena, path_map),
                right: to_node_unresolved(node.right, arena, path_map),
            }))
        }
        Node::ClassStringDisjunction(node) => {
            NodeUnresolved::ClassStringDisjunction(Box::new(ClassStringDisjunctionUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                alternatives: node
                    .alternatives
                    .iter()
                    .map(|&node| to_node_unresolved(node, arena, path_map))
                    .collect(),
            }))
        }
        Node::ClassSubtraction(node) => {
            NodeUnresolved::ClassSubtraction(Box::new(ClassSubtractionUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                left: to_node_unresolved(node.left, arena, path_map),
                right: to_node_unresolved(node.right, arena, path_map),
            }))
        }
        Node::ExpressionCharacterClass(node) => {
            NodeUnresolved::ExpressionCharacterClass(Box::new(ExpressionCharacterClassUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                negate: node.negate,
                expression: to_node_unresolved(node.expression, arena, path_map),
            }))
        }
        Node::Group(node) => NodeUnresolved::Group(Box::new(GroupUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            alternatives: node
                .alternatives
                .iter()
                .map(|&node| to_node_unresolved(node, arena, path_map))
                .collect(),
        })),
        Node::Assertion(node) => NodeUnresolved::Assertion(Box::new(AssertionUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            kind: node.kind,
            negate: node.negate,
            alternatives: node.alternatives.as_ref().map(|alternatives| {
                alternatives
                    .iter()
                    .map(|&node| to_node_unresolved(node, arena, path_map))
                    .collect()
            }),
        })),
        Node::Pattern(node) => NodeUnresolved::Pattern(Box::new(PatternUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            alternatives: node
                .alternatives
                .iter()
                .map(|&node| to_node_unresolved(node, arena, path_map))
                .collect(),
        })),
        Node::Quantifier(node) => NodeUnresolved::Quantifier(Box::new(QuantifierUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            min: node.min,
            max: node.max,
            greedy: node.greedy,
            element: to_node_unresolved(node.element, arena, path_map),
        })),
        Node::RegExpLiteral(node) => {
            NodeUnresolved::RegExpLiteral(Box::new(RegExpLiteralUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                pattern: to_node_unresolved(node.pattern, arena, path_map),
                flags: to_node_unresolved(node.flags, arena, path_map),
            }))
        }
        Node::StringAlternative(node) => {
            NodeUnresolved::StringAlternative(Box::new(StringAlternativeUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                elements: node
                    .elements
                    .iter()
                    .map(|&node| to_node_unresolved(node, arena, path_map))
                    .collect(),
            }))
        }
        Node::Backreference(node) => {
            NodeUnresolved::Backreference(Box::new(BackreferenceUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                ref_: node.ref_.clone(),
                resolved: get_relative_path(node._base._arena_id.unwrap(), node.resolved.unwrap(), path_map),
            }))
        }
        Node::Character(node) => NodeUnresolved::Character(Box::new(CharacterUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            value: node.value,
        })),
        Node::CharacterSet(node) => {
            NodeUnresolved::CharacterSet(Box::new(CharacterSetUnresolved {
                parent: node._base.parent.map(|parent| {
                    get_relative_path(node._base._arena_id.unwrap(), parent, path_map)
                }),
                start: node._base.start,
                end: node._base.end,
                raw: node._base.raw.to_owned(),
                kind: node.kind,
                strings: node.strings,
                key: node.key.clone(),
                value: node.value.clone(),
                negate: node.negate,
            }))
        }
        Node::Flags(node) => NodeUnresolved::Flags(Box::new(FlagsUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            dot_all: node.dot_all,
            global: node.global,
            has_indices: node.has_indices,
            ignore_case: node.ignore_case,
            multiline: node.multiline,
            sticky: node.sticky,
            unicode: node.unicode,
            unicode_sets: node.unicode_sets,
        })),
    }
}

#[derive(Clone)]
pub struct RegExpLiteral {
    _base: NodeBase,
    pub pattern: Id<Node>, /*Pattern*/
    pub flags: Id<Node>,   /*Flags*/
}

fn deserialize_possibly_infinity_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU32 {
        String(String),
        U32(u32),
    }
    let string_or_u32 = StringOrU32::deserialize(deserializer)?;
    Ok(match string_or_u32 {
        StringOrU32::String(value) => {
            // TODO: should handle this better?
            assert!(value == "$$Infinity");
            u32::MAX
        }
        StringOrU32::U32(value) => value,
    })
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RegExpLiteralUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub pattern: NodeUnresolved,
    pub flags: NodeUnresolved,
}

#[derive(Clone)]
pub struct Pattern {
    _base: NodeBase,
    pub alternatives: Vec<Id<Node> /*Alternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PatternUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Alternative {
    _base: NodeBase,
    pub elements: Vec<Id<Node> /*Element*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AlternativeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Group {
    _base: NodeBase,
    pub alternatives: Vec<Id<Node> /*Alternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GroupUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct CapturingGroup {
    _base: NodeBase,
    pub name: Option<Wtf16>,
    pub alternatives: Vec<Id<Node> /*Alternative*/>,
    pub references: Vec<Id<Node> /*Backreference*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CapturingGroupUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub name: Option<Wtf16>,
    pub alternatives: Vec<NodeUnresolved>,
    pub references: Vec<String>,
}

#[derive(Clone)]
pub struct Assertion {
    _base: NodeBase,
    pub kind: AssertionKind,
    pub negate: Option<bool>,
    pub alternatives: Option<Vec<Id<Node> /*Alternative*/>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AssertionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub kind: AssertionKind,
    pub negate: Option<bool>,
    pub alternatives: Option<Vec<NodeUnresolved>>,
}

#[derive(Clone)]
pub struct Quantifier {
    _base: NodeBase,
    pub min: u32,
    pub max: u32,
    pub greedy: bool,
    pub element: Id<Node /*QuantifiableElement*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct QuantifierUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    min: u32,
    #[serde(deserialize_with = "deserialize_possibly_infinity_u32")]
    max: u32,
    greedy: bool,
    element: NodeUnresolved,
}

#[derive(Clone)]
pub struct CharacterClass {
    _base: NodeBase,
    pub unicode_sets: bool,
    pub negate: bool,
    pub elements: Vec<Id<Node> /*CharacterClassElement*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub unicode_sets: bool,
    pub negate: bool,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct CharacterClassRange {
    _base: NodeBase,
    pub min: Id<Node /*Character*/>,
    pub max: Id<Node /*Character*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterClassRangeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub min: NodeUnresolved,
    pub max: NodeUnresolved,
}

#[derive(Clone)]
pub struct CharacterSet {
    _base: NodeBase,
    pub kind: CharacterKind,
    pub strings: Option<bool>,
    pub key: Option<Wtf16>,
    pub value: Option<Wtf16>,
    pub negate: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterSetUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub kind: CharacterKind,
    pub strings: Option<bool>,
    pub key: Option<Wtf16>,
    pub value: Option<Wtf16>,
    pub negate: Option<bool>,
}

#[derive(Clone)]
pub struct ExpressionCharacterClass {
    _base: NodeBase,
    pub negate: bool,
    pub expression: Id<Node /*ClassIntersection | ClassSubtraction*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ExpressionCharacterClassUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub negate: bool,
    pub expression: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassIntersection {
    _base: NodeBase,
    pub left: Id<Node /*ClassIntersection | ClassSetOperand*/>,
    pub right: Id<Node /*ClassSetOperand*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassIntersectionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub left: NodeUnresolved,
    pub right: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassSubtraction {
    _base: NodeBase,
    pub left: Id<Node /*ClassSetOperand | ClassSubtraction*/>,
    pub right: Id<Node /*ClassSetOperand*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassSubtractionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub left: NodeUnresolved,
    pub right: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassStringDisjunction {
    _base: NodeBase,
    pub alternatives: Vec<Id<Node> /*StringAlternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassStringDisjunctionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct StringAlternative {
    _base: NodeBase,
    pub elements: Vec<Id<Node> /*Character*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct StringAlternativeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Character {
    _base: NodeBase,
    pub value: CodePoint,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub value: CodePoint,
}

#[derive(Clone)]
pub struct Backreference {
    _base: NodeBase,
    pub ref_: CapturingGroupKey,
    pub resolved: Option<Id<Node /*CapturingGroup*/>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BackreferenceUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    #[serde(rename = "ref")]
    pub ref_: CapturingGroupKey,
    pub resolved: String,
}

#[derive(Clone)]
pub struct Flags {
    _base: NodeBase,
    pub dot_all: bool,
    pub global: bool,
    pub has_indices: bool,
    pub ignore_case: bool,
    pub multiline: bool,
    pub sticky: bool,
    pub unicode: bool,
    pub unicode_sets: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FlagsUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    pub raw: Wtf16,
    pub dot_all: bool,
    pub global: bool,
    pub has_indices: bool,
    pub ignore_case: bool,
    pub multiline: bool,
    pub sticky: bool,
    pub unicode: bool,
    pub unicode_sets: bool,
}
