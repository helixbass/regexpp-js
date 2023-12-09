use id_arena::Id;

use crate::{
    validator::{
        AnyCharacterKind, CapturingGroupKey, EdgeKind, EscapeCharacterKind, LookaroundKind,
        UnicodePropertyCharacterKind, WordBoundaryKind,
    },
    CodePoint,
};

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

pub trait NodeInterface<'a> {
    fn maybe_parent(&self) -> Option<Id<Node<'a>>>;
    fn parent(&self) -> Id<Node<'a>>;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn raw(&self) -> &'a str;
}

impl<'a> NodeInterface<'a> for Node<'a> {
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

struct NodeBase<'a> {
    // type: Node["type"],
    parent: Option<Id<Node<'a>>>,
    start: usize,
    end: usize,
    raw: &'a str,
}

impl<'a> NodeInterface<'a> for NodeBase<'a> {
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

pub struct RegExpLiteral<'a> {
    _base: NodeBase<'a>,
    pattern: Id<Node<'a>>, /*Pattern*/
    flags: Id<Node<'a>>,   /*Flags*/
}

pub struct Pattern<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

pub struct Alternative<'a> {
    _base: NodeBase<'a>,
    elements: Vec<Id<Node<'a>> /*Element*/>,
}

pub struct Group<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

pub struct CapturingGroup<'a> {
    _base: NodeBase<'a>,
    name: Option<&'a str>,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
    references: Vec<Id<Node<'a>> /*Backreference*/>,
}

pub struct LookaroundAssertion<'a> {
    _base: NodeBase<'a>,
    kind: LookaroundKind,
    negate: bool,
    alternatives: Vec<Id<Node<'a>> /*Alternative*/>,
}

pub struct Quantifier<'a> {
    _base: NodeBase<'a>,
    min: usize,
    max: usize,
    greedy: bool,
    element: Id<Node<'a> /*QuantifiableElement*/>,
}

pub struct CharacterClass<'a> {
    _base: NodeBase<'a>,
    unicode_sets: bool,
    negate: bool,
    elements: Vec<Id<Node<'a>> /*CharacterClassElement*/>,
}

pub struct CharacterClassRange<'a> {
    _base: NodeBase<'a>,
    min: Id<Node<'a> /*Character*/>,
    max: Id<Node<'a> /*Character*/>,
}

pub struct EdgeAssertion<'a> {
    _base: NodeBase<'a>,
    kind: EdgeKind,
}

pub struct WordBoundaryAssertion<'a> {
    _base: NodeBase<'a>,
    kind: WordBoundaryKind,
    negate: bool,
}

pub struct AnyCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: AnyCharacterKind,
}

pub struct EscapeCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: EscapeCharacterKind,
    negate: bool,
}

pub struct UnicodePropertyCharacterSet<'a> {
    _base: NodeBase<'a>,
    kind: UnicodePropertyCharacterKind,
    strings: bool,
    key: &'a str,
    value: Option<&'a str>,
    negate: bool,
}

pub struct ExpressionCharacterClass<'a> {
    _base: NodeBase<'a>,
    negate: bool,
    expression: Id<Node<'a> /*ClassIntersection | ClassSubtraction*/>,
}

pub struct ClassIntersection<'a> {
    _base: NodeBase<'a>,
    left: Id<Node<'a> /*ClassIntersection | ClassSetOperand*/>,
    right: Id<Node<'a> /*ClassSetOperand*/>,
}

pub struct ClassSubtraction<'a> {
    _base: NodeBase<'a>,
    left: Id<Node<'a> /*ClassSetOperand | ClassSubtraction*/>,
    right: Id<Node<'a> /*ClassSetOperand*/>,
}

pub struct ClassStringDisjunction<'a> {
    _base: NodeBase<'a>,
    alternatives: Vec<Id<Node<'a>> /*StringAlternative*/>,
}

pub struct StringAlternative<'a> {
    _base: NodeBase<'a>,
    elements: Vec<Id<Node<'a>> /*Character*/>,
}

pub struct Character<'a> {
    _base: NodeBase<'a>,
    value: CodePoint,
}

pub struct Backreference<'a> {
    _base: NodeBase<'a>,
    ref_: CapturingGroupKey<'a>,
    resolved: Id<Node<'a> /*CapturingGroup*/>,
}

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
