use id_arena::Id;

use crate::{ast::Node, AllArenas};

pub struct RegExpVisitor<'a, THandlers: Handlers> {
    _arena: &'a AllArenas,
    _handlers: &'a THandlers,
}

impl<'a, THandlers: Handlers> RegExpVisitor<'a, THandlers> {
    pub fn new(
        arena: &'a AllArenas,
        handlers: &'a THandlers,
    ) -> Self {
        Self {
            _arena: arena,
            _handlers: handlers,
        }
    }

    pub fn visit(&self, node: Id<Node>) {
        unimplemented!()
    }
}

pub trait Handlers {
    fn on_alternative_enter(&self, node: Id<Node/*Alternative*/>) {}
    fn on_alternative_leave(&self, node: Id<Node/*Alternative*/>) {}
    fn on_assertion_enter(&self, node: Id<Node/*Assertion*/>) {}
    fn on_assertion_leave(&self, node: Id<Node/*Assertion*/>) {}
    fn on_backreference_enter(&self, node: Id<Node/*Backreference*/>) {}
    fn on_backreference_leave(&self, node: Id<Node/*Backreference*/>) {}
    fn on_capturing_group_enter(&self, node: Id<Node/*CapturingGroup*/>) {}
    fn on_capturing_group_leave(&self, node: Id<Node/*CapturingGroup*/>) {}
    fn on_character_enter(&self, node: Id<Node/*Character*/>) {}
    fn on_character_leave(&self, node: Id<Node/*Character*/>) {}
    fn on_character_class_enter(&self, node: Id<Node/*CharacterClass*/>) {}
    fn on_character_class_leave(&self, node: Id<Node/*CharacterClass*/>) {}
    fn on_character_class_range_enter(&self, node: Id<Node/*CharacterClassRange*/>) {}
    fn on_character_class_range_leave(&self, node: Id<Node/*CharacterClassRange*/>) {}
    fn on_character_set_enter(&self, node: Id<Node/*CharacterSet*/>) {}
    fn on_character_set_leave(&self, node: Id<Node/*CharacterSet*/>) {}
    fn on_class_intersection_enter(&self, node: Id<Node/*ClassIntersection*/>) {}
    fn on_class_intersection_leave(&self, node: Id<Node/*ClassIntersection*/>) {}
    fn on_class_string_disjunction_enter(&self, node: Id<Node/*ClassStringDisjunction*/>) {}
    fn on_class_string_disjunction_leave(&self, node: Id<Node/*ClassStringDisjunction*/>) {}
    fn on_class_subtraction_enter(&self, node: Id<Node/*ClassSubtraction*/>) {}
    fn on_class_subtraction_leave(&self, node: Id<Node/*ClassSubtraction*/>) {}
    fn on_expression_character_class_enter(&self, node: Id<Node/*ExpressionCharacterClass*/>) {}
    fn on_expression_character_class_leave(&self, node: Id<Node/*ExpressionCharacterClass*/>) {}
    fn on_flags_enter(&self, node: Id<Node/*Flags*/>) {}
    fn on_flags_leave(&self, node: Id<Node/*Flags*/>) {}
    fn on_group_enter(&self, node: Id<Node/*Group*/>) {}
    fn on_group_leave(&self, node: Id<Node/*Group*/>) {}
    fn on_pattern_enter(&self, node: Id<Node/*Pattern*/>) {}
    fn on_pattern_leave(&self, node: Id<Node/*Pattern*/>) {}
    fn on_quantifier_enter(&self, node: Id<Node/*Quantifier*/>) {}
    fn on_quantifier_leave(&self, node: Id<Node/*Quantifier*/>) {}
    fn on_reg_exp_literal_enter(&self, node: Id<Node/*RegExpLiteral*/>) {}
    fn on_reg_exp_literal_leave(&self, node: Id<Node/*RegExpLiteral*/>) {}
    fn on_string_alternative_enter(&self, node: Id<Node/*StringAlternative*/>) {}
    fn on_string_alternative_leave(&self, node: Id<Node/*StringAlternative*/>) {}
}
