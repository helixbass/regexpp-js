use std::{collections::HashMap, mem};

use id_arena::Id;
use pathdiff::diff_paths;
use serde::{Deserialize, Deserializer};
use serde_bytes::ByteBuf;
use wtf8::Wtf8;

use crate::{
    arena::AllArenas,
    validator::{AssertionKind, CapturingGroupKey, CapturingGroupKeyOwned, CharacterKind},
    CodePoint,
};

struct IdUsize(usize);

impl<'a> From<Id<Node<'a>>> for IdUsize {
    fn from(value: Id<Node<'a>>) -> Self {
        Self(value.index())
    }
}

#[derive(Clone)]
pub enum Node<'a> {
    Alternative(Alternative<'a>),
    CapturingGroup(CapturingGroup<'a>),
    CharacterClass(CharacterClass<'a>),
    CharacterClassRange(CharacterClassRange<'a>),
    ClassIntersection(ClassIntersection<'a>),
    ClassStringDisjunction(ClassStringDisjunction<'a>),
    ClassSubtraction(ClassSubtraction<'a>),
    ExpressionCharacterClass(ExpressionCharacterClass<'a>),
    Group(Group<'a>),
    Assertion(Assertion<'a>),
    Pattern(Pattern<'a>),
    Quantifier(Quantifier<'a>),
    RegExpLiteral(RegExpLiteral<'a>),
    StringAlternative(StringAlternative<'a>),
    Backreference(Backreference<'a>),
    Character(Character<'a>),
    CharacterSet(CharacterSet<'a>),
    Flags(Flags<'a>),
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

pub trait NodeInterface<'a> {
    fn set_arena_id(&mut self, id: Id<Node<'a>>);
    fn arena(&self) -> &AllArenas<'a>;
    fn maybe_parent(&self) -> Option<Id<Node<'a>>>;
    fn parent(&self) -> Id<Node<'a>>;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn raw(&self) -> &'a [u16];
}

impl<'a> NodeInterface<'a> for Node<'a> {
    fn set_arena_id(&mut self, _id: Id<Node<'a>>) {
        todo!()
    }

    fn arena(&self) -> &AllArenas<'a> {
        todo!()
    }

    fn maybe_parent(&self) -> Option<Id<Node<'a>>> {
        todo!()
    }

    fn parent(&self) -> Id<Node<'a>> {
        todo!()
    }

    fn start(&self) -> usize {
        todo!()
    }

    fn end(&self) -> usize {
        todo!()
    }

    fn raw(&self) -> &'a [u16] {
        todo!()
    }
}

fn resolve_location_vec<'a>(
    arena: &AllArenas<'a>,
    nodes: &[Id<Node<'a>>],
    path: &mut Vec<String>,
    path_map: &mut HashMap<Id<Node<'a>>, String>,
) {
    for (index, &node) in nodes.iter().enumerate() {
        path.push(index.to_string());
        resolve_location(arena, node, path, path_map);
        path.pop();
    }
}

pub fn resolve_location<'a>(
    arena: &AllArenas<'a>,
    node: Id<Node<'a>>,
    path: &mut Vec<String>,
    path_map: &mut HashMap<Id<Node<'a>>, String>,
) {
    path_map.insert(node, format!("/{}", path.join("/")));
    match arena.node(node) {
        Node::Alternative(node) => {
            resolve_location_vec(arena, &node.elements, path, path_map);
        }
        Node::CapturingGroup(node) => {
            resolve_location_vec(arena, &node.alternatives, path, path_map);
        }
        Node::CharacterClass(node) => {
            resolve_location_vec(arena, &node.elements, path, path_map);
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
            resolve_location_vec(arena, &node.alternatives, path, path_map);
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
            resolve_location_vec(arena, &node.alternatives, path, path_map);
        }
        Node::Assertion(node) => {
            if let Some(alternatives) = node.alternatives.as_ref() {
                resolve_location_vec(arena, alternatives, path, path_map);
            }
        }
        Node::Pattern(node) => {
            resolve_location_vec(arena, &node.alternatives, path, path_map);
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
            resolve_location_vec(arena, &node.elements, path, path_map);
        }
        _ => (),
    }
}

#[derive(Clone)]
struct NodeBase<'a> {
    _arena_id: Option<Id<Node<'a>>>,
    _arena: *const AllArenas<'a>,
    // type: Node["type"],
    parent: Option<Id<Node<'a>>>,
    start: usize,
    end: usize,
    raw: &'a [u16],
}

impl<'a> NodeInterface<'a> for NodeBase<'a> {
    fn set_arena_id(&mut self, id: Id<Node<'a>>) {
        self._arena_id = Some(id);
    }

    fn arena(&self) -> &AllArenas<'a> {
        unsafe { &*self._arena }
    }

    fn maybe_parent(&self) -> Option<Id<Node<'a>>> {
        self.parent
    }

    fn parent(&self) -> Id<Node<'a>> {
        self.parent.unwrap()
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn raw(&self) -> &'a [u16] {
        self.raw
    }
}

fn get_relative_path<'a>(
    from: Id<Node<'a>>,
    to: Id<Node<'a>>,
    path_map: &HashMap<Id<Node<'a>>, String>,
) -> String {
    let from_path = &path_map[&from];
    let to_path = &path_map[&to];
    let relative = diff_paths(from_path, to_path).unwrap();
    let relative = relative.to_str().unwrap();
    format!(
        "♻️{}",
        relative.strip_suffix('/').unwrap_or(relative),
    )
}

pub fn to_node_unresolved<'a>(
    id: Id<Node<'a>>,
    arena: &AllArenas<'a>,
    path_map: &HashMap<Id<Node<'a>>, String>,
) -> NodeUnresolved {
    match arena.node(id) {
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
                name: node.name.map(ToOwned::to_owned),
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
        Node::RegExpLiteral(node) => NodeUnresolved::RegExpLiteral(Box::new(RegExpLiteralUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            pattern: to_node_unresolved(node.pattern, arena, path_map),
            flags: to_node_unresolved(node.flags, arena, path_map),
        })),
        Node::StringAlternative(node) => NodeUnresolved::StringAlternative(Box::new(StringAlternativeUnresolved {
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
        Node::Backreference(node) => NodeUnresolved::Backreference(Box::new(BackreferenceUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            ref_: node.ref_.into(),
            resolved: get_relative_path(node._base._arena_id.unwrap(), node.resolved, path_map),
        })),
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
        Node::CharacterSet(node) => NodeUnresolved::CharacterSet(Box::new(CharacterSetUnresolved {
            parent: node
                ._base
                .parent
                .map(|parent| get_relative_path(node._base._arena_id.unwrap(), parent, path_map)),
            start: node._base.start,
            end: node._base.end,
            raw: node._base.raw.to_owned(),
            kind: node.kind,
            strings: node.strings,
            key: node.key.map(ToOwned::to_owned),
            value: node.value.map(ToOwned::to_owned),
            negate: node.negate,
        })),
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
pub struct RegExpLiteral<'a> {
    _base: NodeBase<'a>,
    pub pattern: Id<Node<'a>>, /*Pattern*/
    pub flags: Id<Node<'a>>,   /*Flags*/
}

fn deserialize_wtf_16<'de, D>(deserializer: D) -> Result<Vec<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = ByteBuf::deserialize(deserializer)?.into_vec();
    let wtf8 = Wtf8::from_str(unsafe { mem::transmute(&*bytes) });
    Ok(wtf8.to_ill_formed_utf16().collect())
}

fn deserialize_possibly_infinity_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrUsize {
        String(String),
        Usize(usize),
    }
    let string_or_usize = StringOrUsize::deserialize(deserializer)?;
    Ok(match string_or_usize {
        StringOrUsize::String(value) => {
            // TODO: should handle this better?
            assert!(value == "$$Infinity");
            usize::MAX
        }
        StringOrUsize::Usize(value) => value,
    })
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RegExpLiteralUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    // TODO: Encapsulate in an eg Wtf16 type?
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub pattern: NodeUnresolved,
    pub flags: NodeUnresolved,
}

#[derive(Clone)]
pub struct Pattern<'a> {
    _base: NodeBase<'a>,
    pub alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PatternUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Alternative<'a> {
    _base: NodeBase<'a>,
    pub elements: Vec<Id<Node<'a>> /*Element*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AlternativeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Group<'a> {
    _base: NodeBase<'a>,
    pub alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GroupUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct CapturingGroup<'a> {
    _base: NodeBase<'a>,
    pub name: Option<&'a str>,
    pub alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
    pub references: Vec<Id<Node<'a>> /*Backreference*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CapturingGroupUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub name: Option<String>,
    pub alternatives: Vec<NodeUnresolved>,
    pub references: Vec<String>,
}

#[derive(Clone)]
pub struct Assertion<'a> {
    _base: NodeBase<'a>,
    pub kind: AssertionKind,
    pub negate: Option<bool>,
    pub alternatives: Option<Vec<Id<Node<'a>> /*Alternative*/>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AssertionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub kind: AssertionKind,
    pub negate: Option<bool>,
    pub alternatives: Option<Vec<NodeUnresolved>>,
}

#[derive(Clone)]
pub struct Quantifier<'a> {
    _base: NodeBase<'a>,
    pub min: usize,
    pub max: usize,
    pub greedy: bool,
    pub element: Id<Node<'a> /*QuantifiableElement*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct QuantifierUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    min: usize,
    #[serde(deserialize_with = "deserialize_possibly_infinity_usize")]
    max: usize,
    greedy: bool,
    element: NodeUnresolved,
}

#[derive(Clone)]
pub struct CharacterClass<'a> {
    _base: NodeBase<'a>,
    pub unicode_sets: bool,
    pub negate: bool,
    pub elements: Vec<Id<Node<'a>> /*CharacterClassElement*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterClassUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub unicode_sets: bool,
    pub negate: bool,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct CharacterClassRange<'a> {
    _base: NodeBase<'a>,
    pub min: Id<Node<'a> /*Character*/>,
    pub max: Id<Node<'a> /*Character*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterClassRangeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub min: NodeUnresolved,
    pub max: NodeUnresolved,
}

#[derive(Clone)]
pub struct CharacterSet<'a> {
    _base: NodeBase<'a>,
    pub kind: CharacterKind,
    pub strings: Option<bool>,
    pub key: Option<&'a str>,
    pub value: Option<&'a str>,
    pub negate: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterSetUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub kind: CharacterKind,
    pub strings: Option<bool>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub negate: Option<bool>,
}

#[derive(Clone)]
pub struct ExpressionCharacterClass<'a> {
    _base: NodeBase<'a>,
    pub negate: bool,
    pub expression: Id<Node<'a> /*ClassIntersection | ClassSubtraction*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ExpressionCharacterClassUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub negate: bool,
    pub expression: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassIntersection<'a> {
    _base: NodeBase<'a>,
    pub left: Id<Node<'a> /*ClassIntersection | ClassSetOperand*/>,
    pub right: Id<Node<'a> /*ClassSetOperand*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassIntersectionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub left: NodeUnresolved,
    pub right: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassSubtraction<'a> {
    _base: NodeBase<'a>,
    pub left: Id<Node<'a> /*ClassSetOperand | ClassSubtraction*/>,
    pub right: Id<Node<'a> /*ClassSetOperand*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassSubtractionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub left: NodeUnresolved,
    pub right: NodeUnresolved,
}

#[derive(Clone)]
pub struct ClassStringDisjunction<'a> {
    _base: NodeBase<'a>,
    pub alternatives: Vec<Id<Node<'a>> /*StringAlternative*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ClassStringDisjunctionUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub alternatives: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct StringAlternative<'a> {
    _base: NodeBase<'a>,
    pub elements: Vec<Id<Node<'a>> /*Character*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct StringAlternativeUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub elements: Vec<NodeUnresolved>,
}

#[derive(Clone)]
pub struct Character<'a> {
    _base: NodeBase<'a>,
    pub value: CodePoint,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CharacterUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub value: CodePoint,
}

#[derive(Clone)]
pub struct Backreference<'a> {
    _base: NodeBase<'a>,
    pub ref_: CapturingGroupKey<'a>,
    pub resolved: Id<Node<'a> /*CapturingGroup*/>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BackreferenceUnresolved {
    pub parent: Option<String>,
    pub start: usize,
    pub end: usize,
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    #[serde(rename = "ref")]
    pub ref_: CapturingGroupKeyOwned,
    pub resolved: String,
}

#[derive(Clone)]
pub struct Flags<'a> {
    _base: NodeBase<'a>,
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
    #[serde(deserialize_with = "deserialize_wtf_16")]
    pub raw: Vec<u16>,
    pub dot_all: bool,
    pub global: bool,
    pub has_indices: bool,
    pub ignore_case: bool,
    pub multiline: bool,
    pub sticky: bool,
    pub unicode: bool,
    pub unicode_sets: bool,
}
