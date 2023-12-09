use id_arena::Id;
use serde::Serialize;

use crate::{
    validator::{
        AnyCharacterKind, CapturingGroupKey, EdgeKind, EscapeCharacterKind, LookaroundKind,
        UnicodePropertyCharacterKind, WordBoundaryKind, CapturingGroupKeyOwned,
    },
    CodePoint, arena::AllArenas,
};

#[derive(Serialize)]
struct IdSerializable(usize);

impl<'a> From<Id<Node<'a>>> for IdSerializable {
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
    LookaroundAssertion(LookaroundAssertion<'a>),
    Pattern(Pattern<'a>),
    Quantifier(Quantifier<'a>),
    RegExpLiteral(RegExpLiteral<'a>),
    StringAlternative(StringAlternative<'a>),
    Backreference(Backreference<'a>),
    EdgeAssertion(EdgeAssertion<'a>),
    WordBoundaryAssertion(WordBoundaryAssertion<'a>),
    Character(Character<'a>),
    AnyCharacterSet(AnyCharacterSet<'a>),
    EscapeCharacterSet(EscapeCharacterSet<'a>),
    UnicodePropertyCharacterSet(UnicodePropertyCharacterSet<'a>),
    Flags(Flags<'a>),
}

#[derive(Serialize)]
enum NodeSerializable {
    Alternative(Box<AlternativeSerializable>),
    CapturingGroup(Box<CapturingGroupSerializable>),
    CharacterClass(Box<CharacterClassSerializable>),
    CharacterClassRange(Box<CharacterClassRangeSerializable>),
    ClassIntersection(Box<ClassIntersectionSerializable>),
    ClassStringDisjunction(Box<ClassStringDisjunctionSerializable>),
    ClassSubtraction(Box<ClassSubtractionSerializable>),
    ExpressionCharacterClass(Box<ExpressionCharacterClassSerializable>),
    Group(Box<GroupSerializable>),
    LookaroundAssertion(Box<LookaroundAssertionSerializable>),
    Pattern(Box<PatternSerializable>),
    Quantifier(Box<QuantifierSerializable>),
    RegExpLiteral(Box<RegExpLiteralSerializable>),
    StringAlternative(Box<StringAlternativeSerializable>),
    Backreference(Box<BackreferenceSerializable>),
    EdgeAssertion(Box<EdgeAssertionSerializable>),
    WordBoundaryAssertion(Box<WordBoundaryAssertionSerializable>),
    Character(Box<CharacterSerializable>),
    AnyCharacterSet(Box<AnyCharacterSetSerializable>),
    EscapeCharacterSet(Box<EscapeCharacterSetSerializable>),
    UnicodePropertyCharacterSet(Box<UnicodePropertyCharacterSetSerializable>),
    Flags(Box<FlagsSerializable>),
}

impl<'a> From<Node<'a>> for NodeSerializable {
    fn from(value: Node<'a>) -> Self {
        unimplemented!()
    }
}

pub trait NodeInterface<'a> {
    fn set_arena_id(&mut self, id: Id<Node<'a>>);
    fn arena(&self) -> &AllArenas<'a>;
    fn maybe_parent(&self) -> Option<Id<Node<'a>>>;
    fn parent(&self) -> Id<Node<'a>>;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn raw(&self) -> &'a str;
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

    fn raw(&self) -> &'a str {
        todo!()
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
    raw: &'a str,
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

    fn raw(&self) -> &'a str {
        self.raw
    }
}

#[derive(Serialize)]
struct NodeBaseSerializable {
    id: IdSerializable,
    parent: Option<IdSerializable>,
    start: usize,
    end: usize,
    raw: String,
}

impl<'a> From<NodeBase<'a>> for NodeBaseSerializable {
    fn from(value: NodeBase<'a>) -> Self {
        Self {
            id: value._arena_id.unwrap().into(),
            parent: value.parent.map(Into::into),
            start: value.start,
            end: value.end,
            raw: value.raw.to_owned(),
        }
    }
}

impl<'a> Serialize for NodeBase<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        NodeBaseSerializable::from(self.clone()).serialize(serializer)
    }
}

fn to_node_serializable<'a>(
    id: Id<Node<'a>>,
    arena: &AllArenas<'a>,
) -> NodeSerializable {
    arena.node(id).clone().into()
}

#[derive(Clone)]
pub struct RegExpLiteral<'a> {
    _base: NodeBase<'a>,
    pattern: Id<Node<'a>>, /*Pattern*/
    flags: Id<Node<'a>>,   /*Flags*/
}

#[derive(Serialize)]
struct RegExpLiteralSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    pattern: NodeSerializable,
    flags: NodeSerializable,
}

impl<'a> From<RegExpLiteral<'a>> for RegExpLiteralSerializable {
    fn from(value: RegExpLiteral<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            pattern: to_node_serializable(value.pattern, value._base.arena()),
            flags: to_node_serializable(value.flags, value._base.arena()),
        }
    }
}

impl<'a> Serialize for RegExpLiteral<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        RegExpLiteralSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Pattern<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

#[derive(Serialize)]
struct PatternSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    alternatives: Vec<NodeSerializable>,
}

impl<'a> From<Pattern<'a>> for PatternSerializable {
    fn from(value: Pattern<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            alternatives: value.alternatives.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for Pattern<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        PatternSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Alternative<'a> {
    _base: NodeBase<'a>,
    elements: Vec<Id<Node<'a>> /*Element*/>,
}

#[derive(Serialize)]
struct AlternativeSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    elements: Vec<NodeSerializable>,
}

impl<'a> From<Alternative<'a>> for AlternativeSerializable {
    fn from(value: Alternative<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            elements: value.elements.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for Alternative<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        AlternativeSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Group<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

#[derive(Serialize)]
struct GroupSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    alternatives: Vec<NodeSerializable>,
}

impl<'a> From<Group<'a>> for GroupSerializable {
    fn from(value: Group<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            alternatives: value.alternatives.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for Group<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        GroupSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct CapturingGroup<'a> {
    _base: NodeBase<'a>,
    name: Option<&'a str>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
    references: Vec<Id<Node<'a>> /*Backreference*/>,
}

#[derive(Serialize)]
struct CapturingGroupSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    name: Option<String>,
    alternatives: Vec<NodeSerializable>,
    references: Vec<IdSerializable>,
}

impl<'a> From<CapturingGroup<'a>> for CapturingGroupSerializable {
    fn from(value: CapturingGroup<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            name: value.name.map(ToOwned::to_owned),
            alternatives: value.alternatives.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
            references: value.references.iter().copied().map(Into::into).collect(),
        }
    }
}

impl<'a> Serialize for CapturingGroup<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        CapturingGroupSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct LookaroundAssertion<'a> {
    _base: NodeBase<'a>,
    kind: LookaroundKind,
    negate: bool,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

#[derive(Serialize)]
struct LookaroundAssertionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: LookaroundKind,
    negate: bool,
    alternatives: Vec<NodeSerializable>,
}

impl<'a> From<LookaroundAssertion<'a>> for LookaroundAssertionSerializable {
    fn from(value: LookaroundAssertion<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            kind: value.kind,
            negate: value.negate,
            alternatives: value.alternatives.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for LookaroundAssertion<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        LookaroundAssertionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Quantifier<'a> {
    _base: NodeBase<'a>,
    min: usize,
    max: usize,
    greedy: bool,
    element: Id<Node<'a> /*QuantifiableElement*/>,
}

#[derive(Serialize)]
struct QuantifierSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    min: usize,
    max: usize,
    greedy: bool,
    element: NodeSerializable,
}

impl<'a> From<Quantifier<'a>> for QuantifierSerializable {
    fn from(value: Quantifier<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            min: value.min,
            max: value.max,
            greedy: value.greedy,
            element: to_node_serializable(value.element, value._base.arena()),
        }
    }
}

impl<'a> Serialize for Quantifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        QuantifierSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct CharacterClass<'a> {
    _base: NodeBase<'a>,
    unicode_sets: bool,
    negate: bool,
    elements: Vec<Id<Node<'a>> /*CharacterClassElement*/>,
}

#[derive(Serialize)]
struct CharacterClassSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    unicode_sets: bool,
    negate: bool,
    elements: Vec<NodeSerializable>,
}

impl<'a> From<CharacterClass<'a>> for CharacterClassSerializable {
    fn from(value: CharacterClass<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            unicode_sets: value.unicode_sets,
            negate: value.negate,
            elements: value.elements.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for CharacterClass<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        CharacterClassSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct CharacterClassRange<'a> {
    _base: NodeBase<'a>,
    min: Id<Node<'a> /*Character*/>,
    max: Id<Node<'a> /*Character*/>,
}

#[derive(Serialize)]
struct CharacterClassRangeSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    min: NodeSerializable,
    max: NodeSerializable,
}

impl<'a> From<CharacterClassRange<'a>> for CharacterClassRangeSerializable {
    fn from(value: CharacterClassRange<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            min: to_node_serializable(value.min, value._base.arena()),
            max: to_node_serializable(value.max, value._base.arena()),
        }
    }
}

impl<'a> Serialize for CharacterClassRange<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        CharacterClassRangeSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct EdgeAssertion<'a> {
    _base: NodeBase<'a>,
    kind: EdgeKind,
}

#[derive(Serialize)]
struct EdgeAssertionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: EdgeKind,
}

impl<'a> From<EdgeAssertion<'a>> for EdgeAssertionSerializable {
    fn from(value: EdgeAssertion<'a>) -> Self {
        Self {
            _base: value._base.into(),
            kind: value.kind,
        }
    }
}

impl<'a> Serialize for EdgeAssertion<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        EdgeAssertionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct WordBoundaryAssertion<'a> {
    _base: NodeBase<'a>,
    kind: WordBoundaryKind,
    negate: bool,
}

#[derive(Serialize)]
struct WordBoundaryAssertionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: WordBoundaryKind,
    negate: bool,
}

impl<'a> From<WordBoundaryAssertion<'a>> for WordBoundaryAssertionSerializable {
    fn from(value: WordBoundaryAssertion<'a>) -> Self {
        Self {
            _base: value._base.into(),
            kind: value.kind,
            negate: value.negate,
        }
    }
}

impl<'a> Serialize for WordBoundaryAssertion<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        WordBoundaryAssertionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct AnyCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: AnyCharacterKind,
}

#[derive(Serialize)]
struct AnyCharacterSetSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: AnyCharacterKind,
}

impl<'a> From<AnyCharacterSet<'a>> for AnyCharacterSetSerializable {
    fn from(value: AnyCharacterSet<'a>) -> Self {
        Self {
            _base: value._base.into(),
            kind: value.kind,
        }
    }
}

impl<'a> Serialize for AnyCharacterSet<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        AnyCharacterSetSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct EscapeCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: EscapeCharacterKind,
    negate: bool,
}

#[derive(Serialize)]
struct EscapeCharacterSetSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: EscapeCharacterKind,
    negate: bool,
}

impl<'a> From<EscapeCharacterSet<'a>> for EscapeCharacterSetSerializable {
    fn from(value: EscapeCharacterSet<'a>) -> Self {
        Self {
            _base: value._base.into(),
            kind: value.kind,
            negate: value.negate,
        }
    }
}

impl<'a> Serialize for EscapeCharacterSet<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        EscapeCharacterSetSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct UnicodePropertyCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: UnicodePropertyCharacterKind,
    strings: bool,
    key: &'a str,
    value: Option<&'a str>,
    negate: bool,
}

#[derive(Serialize)]
struct UnicodePropertyCharacterSetSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    kind: UnicodePropertyCharacterKind,
    strings: bool,
    key: String,
    value: Option<String>,
    negate: bool,
}

impl<'a> From<UnicodePropertyCharacterSet<'a>> for UnicodePropertyCharacterSetSerializable {
    fn from(value: UnicodePropertyCharacterSet<'a>) -> Self {
        Self {
            _base: value._base.into(),
            kind: value.kind,
            strings: value.strings,
            key: value.key.to_owned(),
            value: value.value.map(ToOwned::to_owned),
            negate: value.negate,
        }
    }
}

impl<'a> Serialize for UnicodePropertyCharacterSet<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        UnicodePropertyCharacterSetSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct ExpressionCharacterClass<'a> {
    _base: NodeBase<'a>,
    negate: bool,
    expression: Id<Node<'a> /*ClassIntersection | ClassSubtraction*/>,
}

#[derive(Serialize)]
struct ExpressionCharacterClassSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    negate: bool,
    expression: NodeSerializable,
}

impl<'a> From<ExpressionCharacterClass<'a>> for ExpressionCharacterClassSerializable {
    fn from(value: ExpressionCharacterClass<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            negate: value.negate,
            expression: to_node_serializable(value.expression, value._base.arena()),
        }
    }
}

impl<'a> Serialize for ExpressionCharacterClass<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        ExpressionCharacterClassSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct ClassIntersection<'a> {
    _base: NodeBase<'a>,
    left: Id<Node<'a> /*ClassIntersection | ClassSetOperand*/>,
    right: Id<Node<'a> /*ClassSetOperand*/>,
}

#[derive(Serialize)]
struct ClassIntersectionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    left: NodeSerializable,
    right: NodeSerializable,
}

impl<'a> From<ClassIntersection<'a>> for ClassIntersectionSerializable {
    fn from(value: ClassIntersection<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            left: to_node_serializable(value.left, value._base.arena()),
            right: to_node_serializable(value.right, value._base.arena()),
        }
    }
}

impl<'a> Serialize for ClassIntersection<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        ClassIntersectionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct ClassSubtraction<'a> {
    _base: NodeBase<'a>,
    left: Id<Node<'a> /*ClassSetOperand | ClassSubtraction*/>,
    right: Id<Node<'a> /*ClassSetOperand*/>,
}

#[derive(Serialize)]
struct ClassSubtractionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    left: NodeSerializable,
    right: NodeSerializable,
}

impl<'a> From<ClassSubtraction<'a>> for ClassSubtractionSerializable {
    fn from(value: ClassSubtraction<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            left: to_node_serializable(value.left, value._base.arena()),
            right: to_node_serializable(value.right, value._base.arena()),
        }
    }
}

impl<'a> Serialize for ClassSubtraction<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        ClassSubtractionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct ClassStringDisjunction<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*StringAlternative*/>,
}

#[derive(Serialize)]
struct ClassStringDisjunctionSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    alternatives: Vec<NodeSerializable>,
}

impl<'a> From<ClassStringDisjunction<'a>> for ClassStringDisjunctionSerializable {
    fn from(value: ClassStringDisjunction<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            alternatives: value.alternatives.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for ClassStringDisjunction<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        ClassStringDisjunctionSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct StringAlternative<'a> {
    _base: NodeBase<'a>,
    elements: Vec<Id<Node<'a>> /*Character*/>,
}

#[derive(Serialize)]
struct StringAlternativeSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    elements: Vec<NodeSerializable>,
}

impl<'a> From<StringAlternative<'a>> for StringAlternativeSerializable {
    fn from(value: StringAlternative<'a>) -> Self {
        Self {
            _base: value._base.clone().into(),
            elements: value.elements.iter().map(|&id| to_node_serializable(id, value._base.arena())).collect(),
        }
    }
}

impl<'a> Serialize for StringAlternative<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        StringAlternativeSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Character<'a> {
    _base: NodeBase<'a>,
    value: CodePoint,
}

#[derive(Serialize)]
struct CharacterSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    value: CodePoint,
}

impl<'a> From<Character<'a>> for CharacterSerializable {
    fn from(value: Character<'a>) -> Self {
        Self {
            _base: value._base.into(),
            value: value.value,
        }
    }
}

impl<'a> Serialize for Character<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        CharacterSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Backreference<'a> {
    _base: NodeBase<'a>,
    ref_: CapturingGroupKey<'a>,
    resolved: Id<Node<'a> /*CapturingGroup*/>,
}

#[derive(Serialize)]
struct BackreferenceSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    ref_: CapturingGroupKeyOwned,
    resolved: IdSerializable,
}

impl<'a> From<Backreference<'a>> for BackreferenceSerializable {
    fn from(value: Backreference<'a>) -> Self {
        Self {
            _base: value._base.into(),
            ref_: value.ref_.into(),
            resolved: value.resolved.into(),
        }
    }
}

impl<'a> Serialize for Backreference<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        BackreferenceSerializable::from(self.clone()).serialize(serializer)
    }
}

#[derive(Clone)]
pub struct Flags<'a> {
    _base: NodeBase<'a>,
    dot_all: bool,
    global: bool,
    has_indices: bool,
    ignore_case: bool,
    multiline: bool,
    sticky: bool,
    unicode: bool,
    unicode_sets: bool,
}

#[derive(Serialize)]
struct FlagsSerializable {
    #[serde(flatten)]
    _base: NodeBaseSerializable,
    dot_all: bool,
    global: bool,
    has_indices: bool,
    ignore_case: bool,
    multiline: bool,
    sticky: bool,
    unicode: bool,
    unicode_sets: bool,
}

impl<'a> From<Flags<'a>> for FlagsSerializable {
    fn from(value: Flags<'a>) -> Self {
        Self {
            _base: value._base.into(),
            dot_all: value.dot_all,
            global: value.global,
            has_indices: value.has_indices,
            ignore_case: value.ignore_case,
            multiline: value.multiline,
            sticky: value.sticky,
            unicode: value.unicode,
            unicode_sets: value.unicode_sets,
        }
    }
}

impl<'a> Serialize for Flags<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        FlagsSerializable::from(self.clone()).serialize(serializer)
    }
}
