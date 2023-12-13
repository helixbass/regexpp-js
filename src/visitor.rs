use id_arena::Id;
use squalid::EverythingExt;

use crate::{ast::Node, AllArenas};

pub struct RegExpVisitor<'a, THandlers: Handlers> {
    _arena: &'a AllArenas,
    _handlers: &'a THandlers,
}

impl<'a, THandlers: Handlers> RegExpVisitor<'a, THandlers> {
    pub fn new(arena: &'a AllArenas, handlers: &'a THandlers) -> Self {
        Self {
            _arena: arena,
            _handlers: handlers,
        }
    }

    pub fn visit(&self, node: Id<Node>) {
        match &*self._arena.node(node) {
            Node::Alternative(_) => self.visit_alternative(node),
            Node::Assertion(_) => self.visit_assertion(node),
            Node::Backreference(_) => self.visit_backreference(node),
            Node::CapturingGroup(_) => self.visit_capturing_group(node),
            Node::Character(_) => self.visit_character(node),
            Node::CharacterClass(_) => self.visit_character_class(node),
            Node::CharacterClassRange(_) => self.visit_character_class_range(node),
            Node::CharacterSet(_) => self.visit_character_set(node),
            Node::ClassIntersection(_) => self.visit_class_intersection(node),
            Node::ClassStringDisjunction(_) => self.visit_class_string_disjunction(node),
            Node::ClassSubtraction(_) => self.visit_class_subtraction(node),
            Node::ExpressionCharacterClass(_) => self.visit_expression_character_class(node),
            Node::Flags(_) => self.visit_flags(node),
            Node::Group(_) => self.visit_group(node),
            Node::Pattern(_) => self.visit_pattern(node),
            Node::Quantifier(_) => self.visit_quantifier(node),
            Node::RegExpLiteral(_) => self.visit_reg_exp_literal(node),
            Node::StringAlternative(_) => self.visit_string_alternative(node),
        }
    }

    fn visit_alternative(&self, node: Id<Node>) {
        self._handlers.on_alternative_enter(node);
        // TODO: should probably not hold a RefCell borrow while
        // invoking the handlers in case someonw wants to mutably
        // borrow from the arena in them?
        self._arena
            .node(node)
            .as_alternative()
            .elements
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_alternative_leave(node);
    }

    fn visit_assertion(&self, node: Id<Node>) {
        self._handlers.on_assertion_enter(node);
        if let Some(alternatives) = self._arena.node(node).as_assertion().alternatives.as_ref() {
            alternatives.iter().for_each(|&node| self.visit(node));
        }
        self._handlers.on_assertion_leave(node);
    }

    fn visit_backreference(&self, node: Id<Node>) {
        self._handlers.on_backreference_enter(node);
        self._handlers.on_backreference_leave(node);
    }

    fn visit_capturing_group(&self, node: Id<Node>) {
        self._handlers.on_capturing_group_enter(node);
        self._arena
            .node(node)
            .as_capturing_group()
            .alternatives
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_capturing_group_leave(node);
    }

    fn visit_character(&self, node: Id<Node>) {
        self._handlers.on_character_enter(node);
        self._handlers.on_character_leave(node);
    }

    fn visit_character_class(&self, node: Id<Node>) {
        self._handlers.on_character_class_enter(node);
        self._arena
            .node(node)
            .as_character_class()
            .elements
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_character_class_leave(node);
    }

    fn visit_character_class_range(&self, node: Id<Node>) {
        self._handlers.on_character_class_range_enter(node);
        self._arena
            .node(node)
            .as_character_class_range()
            .thrush(|node| {
                self.visit_character(node.min);
                self.visit_character(node.max);
            });
        self._handlers.on_character_class_range_leave(node);
    }

    fn visit_character_set(&self, node: Id<Node>) {
        self._handlers.on_character_set_enter(node);
        self._handlers.on_character_set_leave(node);
    }

    fn visit_class_intersection(&self, node: Id<Node>) {
        self._handlers.on_class_intersection_enter(node);
        self._arena
            .node(node)
            .as_class_intersection()
            .thrush(|node| {
                self.visit(node.left);
                self.visit(node.right);
            });
        self._handlers.on_class_intersection_leave(node);
    }

    fn visit_class_string_disjunction(&self, node: Id<Node>) {
        self._handlers.on_class_string_disjunction_enter(node);
        self._arena
            .node(node)
            .as_class_string_disjunction()
            .alternatives
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_class_string_disjunction_leave(node);
    }

    fn visit_class_subtraction(&self, node: Id<Node>) {
        self._handlers.on_class_subtraction_enter(node);
        self._arena
            .node(node)
            .as_class_subtraction()
            .thrush(|node| {
                self.visit(node.left);
                self.visit(node.right);
            });
        self._handlers.on_class_subtraction_leave(node);
    }

    fn visit_expression_character_class(&self, node: Id<Node>) {
        self._handlers.on_expression_character_class_enter(node);
        self.visit(
            self._arena
                .node(node)
                .as_expression_character_class()
                .expression,
        );
        self._handlers.on_expression_character_class_leave(node);
    }

    fn visit_flags(&self, node: Id<Node>) {
        self._handlers.on_flags_enter(node);
        self._handlers.on_flags_leave(node);
    }

    fn visit_group(&self, node: Id<Node>) {
        self._handlers.on_group_enter(node);
        self._arena
            .node(node)
            .as_group()
            .alternatives
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_group_leave(node);
    }

    fn visit_pattern(&self, node: Id<Node>) {
        self._handlers.on_pattern_enter(node);
        self._arena
            .node(node)
            .as_pattern()
            .alternatives
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_pattern_leave(node);
    }

    fn visit_quantifier(&self, node: Id<Node>) {
        self._handlers.on_quantifier_enter(node);
        self.visit(self._arena.node(node).as_quantifier().element);
        self._handlers.on_quantifier_leave(node);
    }

    fn visit_reg_exp_literal(&self, node: Id<Node>) {
        self._handlers.on_reg_exp_literal_enter(node);
        self._arena.node(node).as_reg_exp_literal().thrush(|node| {
            self.visit_pattern(node.pattern);
            self.visit_flags(node.flags);
        });
        self._handlers.on_reg_exp_literal_leave(node);
    }

    fn visit_string_alternative(&self, node: Id<Node>) {
        self._handlers.on_string_alternative_enter(node);
        self._arena
            .node(node)
            .as_string_alternative()
            .elements
            .iter()
            .for_each(|&node| self.visit(node));
        self._handlers.on_string_alternative_leave(node);
    }
}

#[allow(unused_variables)]
pub trait Handlers {
    fn on_alternative_enter(&self, node: Id<Node /*Alternative*/>) {}
    fn on_alternative_leave(&self, node: Id<Node /*Alternative*/>) {}
    fn on_assertion_enter(&self, node: Id<Node /*Assertion*/>) {}
    fn on_assertion_leave(&self, node: Id<Node /*Assertion*/>) {}
    fn on_backreference_enter(&self, node: Id<Node /*Backreference*/>) {}
    fn on_backreference_leave(&self, node: Id<Node /*Backreference*/>) {}
    fn on_capturing_group_enter(&self, node: Id<Node /*CapturingGroup*/>) {}
    fn on_capturing_group_leave(&self, node: Id<Node /*CapturingGroup*/>) {}
    fn on_character_enter(&self, node: Id<Node /*Character*/>) {}
    fn on_character_leave(&self, node: Id<Node /*Character*/>) {}
    fn on_character_class_enter(&self, node: Id<Node /*CharacterClass*/>) {}
    fn on_character_class_leave(&self, node: Id<Node /*CharacterClass*/>) {}
    fn on_character_class_range_enter(&self, node: Id<Node /*CharacterClassRange*/>) {}
    fn on_character_class_range_leave(&self, node: Id<Node /*CharacterClassRange*/>) {}
    fn on_character_set_enter(&self, node: Id<Node /*CharacterSet*/>) {}
    fn on_character_set_leave(&self, node: Id<Node /*CharacterSet*/>) {}
    fn on_class_intersection_enter(&self, node: Id<Node /*ClassIntersection*/>) {}
    fn on_class_intersection_leave(&self, node: Id<Node /*ClassIntersection*/>) {}
    fn on_class_string_disjunction_enter(&self, node: Id<Node /*ClassStringDisjunction*/>) {}
    fn on_class_string_disjunction_leave(&self, node: Id<Node /*ClassStringDisjunction*/>) {}
    fn on_class_subtraction_enter(&self, node: Id<Node /*ClassSubtraction*/>) {}
    fn on_class_subtraction_leave(&self, node: Id<Node /*ClassSubtraction*/>) {}
    fn on_expression_character_class_enter(&self, node: Id<Node /*ExpressionCharacterClass*/>) {}
    fn on_expression_character_class_leave(&self, node: Id<Node /*ExpressionCharacterClass*/>) {}
    fn on_flags_enter(&self, node: Id<Node /*Flags*/>) {}
    fn on_flags_leave(&self, node: Id<Node /*Flags*/>) {}
    fn on_group_enter(&self, node: Id<Node /*Group*/>) {}
    fn on_group_leave(&self, node: Id<Node /*Group*/>) {}
    fn on_pattern_enter(&self, node: Id<Node /*Pattern*/>) {}
    fn on_pattern_leave(&self, node: Id<Node /*Pattern*/>) {}
    fn on_quantifier_enter(&self, node: Id<Node /*Quantifier*/>) {}
    fn on_quantifier_leave(&self, node: Id<Node /*Quantifier*/>) {}
    fn on_reg_exp_literal_enter(&self, node: Id<Node /*RegExpLiteral*/>) {}
    fn on_reg_exp_literal_leave(&self, node: Id<Node /*RegExpLiteral*/>) {}
    fn on_string_alternative_enter(&self, node: Id<Node /*StringAlternative*/>) {}
    fn on_string_alternative_leave(&self, node: Id<Node /*StringAlternative*/>) {}
}
