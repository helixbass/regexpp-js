#![allow(clippy::into_iter_on_ref)]

/// Derived from [regexpp](https://github.com/eslint-community/regexpp)

mod arena;
mod ast;
mod ecma_versions;
mod parser;
mod reader;
mod regexp_syntax_error;
#[cfg(test)]
mod test;
mod unicode;
pub mod validator;
pub mod visitor;
mod wtf16;

pub use arena::AllArenas;
pub use ast::{Node, NodeInterface};
pub use ecma_versions::EcmaVersion;
use id_arena::Id;
pub use parser::RegExpParser;
pub use reader::{CodePoint, Reader};
pub use regexp_syntax_error::RegExpSyntaxError;
pub use validator::{RegExpValidator, ValidatePatternFlags};
use visitor::RegExpVisitor;
pub use wtf16::Wtf16;

pub extern crate id_arena;

pub type Result<T> = std::result::Result<T, RegExpSyntaxError>;

pub fn parse_reg_exp_literal(
    source: &[u16],
    options: Option<parser::Options>,
    arena: &AllArenas,
) -> Result<Id<Node> /*AST.RegExpLiteral*/> {
    RegExpParser::new(arena, options).parse_literal(source, None, None)
}

pub fn visit_reg_exp_ast(node: Id<Node>, handlers: &impl visitor::Handlers, arena: &AllArenas) {
    RegExpVisitor::new(arena, handlers).visit(node);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use itertools::Itertools;
    use regex::Captures;
    use speculoos::prelude::*;
    use squalid::regex;

    use super::*;

    use crate::{
        ast::{resolve_location, to_node_unresolved, NodeUnresolved},
        test::fixtures::{
            self,
            parser::literal::{self, AstOrError},
        },
        unicode::{
            is_line_terminator, ASTERISK, LEFT_SQUARE_BRACKET, REVERSE_SOLIDUS,
            RIGHT_SQUARE_BRACKET, SOLIDUS,
        },
        validator::{bmp_char_to_utf_16, ValidatePatternFlags},
    };

    #[test]
    fn test_parse_reg_exp_literal_fixtures() {
        fn generate_ast(
            source: &[u16],
            options: parser::Options,
            arena: &AllArenas,
        ) -> NodeUnresolved {
            let ast = parse_reg_exp_literal(source, Some(options), arena).unwrap();
            let mut path: Vec<String> = Default::default();
            let mut path_map: HashMap<Id<Node>, String> = Default::default();
            resolve_location(arena, ast, &mut path, &mut path_map);
            to_node_unresolved(ast, arena, &path_map)
        }

        for (filename, fixture) in &*literal::FIXTURES_DATA {
            let options = fixture.options;
            if filename.to_str().unwrap().contains("-valid") {
                for result in fixture.patterns.values() {
                    assert_that!(&matches!(result, AstOrError::Ast(_),)).is_true();
                }
            } else if filename.to_str().unwrap().contains("-invalid") {
                for result in fixture.patterns.values() {
                    assert_that!(&matches!(result, AstOrError::Error(_),)).is_true();
                }
            }

            for (source, result) in &fixture.patterns {
                let source: Wtf16 = (&**source).into();
                let arena = AllArenas::default();
                match result {
                    AstOrError::Ast(expected) => {
                        let actual = generate_ast(&source, options, &arena);
                        assert_that!(&actual).is_equal_to(expected);
                    }
                    AstOrError::Error(expected) => {
                        let err =
                            parse_reg_exp_literal(&source, Some(options), &arena).unwrap_err();
                        assert_that!(&err).is_equal_to(expected);

                        assert_that!(&&expected.message[..27])
                            .is_equal_to(&"Invalid regular expression:");

                        let mut validator = RegExpValidator::new(Some(Rc::new(options)));
                        if let Some(extracted) = extract_pattern_and_flags(&source, &mut validator)
                        {
                            let error = validator
                                .validate_pattern(
                                    &extracted.pattern,
                                    None,
                                    None,
                                    Some(ValidatePatternFlags {
                                        unicode: Some(
                                            extracted.flags.contains(&bmp_char_to_utf_16('u')),
                                        ),
                                        unicode_sets: Some(
                                            extracted.flags.contains(&bmp_char_to_utf_16('v')),
                                        ),
                                    }),
                                )
                                .unwrap_err();
                            let expected_message = regex!(r#"/([a-z]+?):"#)
                                .replace(&expected.message, |captures: &Captures| {
                                    format!(
                                        "/{}:",
                                        regex!(r#"[^uv]"#).replace_all(&captures[1], "")
                                    )
                                })
                                .into_owned();
                            let expected_index = expected.index - 1;
                            assert_that!(&error.message).is_equal_to(&expected_message);
                            assert_that!(&error.index).is_equal_to(expected_index);
                        }
                    }
                }
            }
        }
    }

    struct PatternAndFlags {
        pattern: Wtf16,
        flags: Wtf16,
    }

    fn extract_pattern_and_flags(
        source: &Wtf16,
        validator: &mut RegExpValidator,
    ) -> Option<PatternAndFlags> {
        let mut in_class = false;
        let mut escaped = false;

        let mut chars = source.split_code_points().collect_vec();

        if chars.get(0) != Some(&vec![bmp_char_to_utf_16('/')].into()) {
            return None;
        }
        chars.remove(0);

        let mut pattern: Vec<Wtf16> = Default::default();

        let mut first = true;
        loop {
            if chars.is_empty() {
                return None;
            }
            let char = chars.remove(0);
            let cp: CodePoint = char[0].into();
            if is_line_terminator(cp) {
                return None;
            }
            if escaped {
                escaped = false;
            } else if cp == REVERSE_SOLIDUS {
                escaped = true;
            } else if cp == LEFT_SQUARE_BRACKET {
                in_class = true;
            } else if cp == RIGHT_SQUARE_BRACKET {
                in_class = false;
            } else if cp == ASTERISK && first {
                return None;
            } else if cp == SOLIDUS && !in_class {
                break;
            }
            pattern.push(char);
            first = false;
        }

        let flags: Wtf16 = chars
            .iter()
            .flat_map(|ch| (**ch).clone().into_iter())
            .collect_vec()
            .into();
        if pattern.is_empty() {
            return None;
        }

        validator.validate_flags(&flags, None, None).ok()?;

        Some(PatternAndFlags {
            pattern: pattern
                .iter()
                .flat_map(|ch| (**ch).clone().into_iter())
                .collect_vec()
                .into(),
            flags,
        })
    }

    #[test]
    fn test_visit_reg_exp_ast() {
        fn generate_ast(source: &[u16], options: parser::Options, arena: &AllArenas) -> Id<Node> {
            parse_reg_exp_literal(source, Some(options), arena).unwrap()
        }

        for (_filename, fixture) in &*fixtures::visitor::FIXTURES_DATA {
            let options = fixture.options;
            for (source, expected) in &fixture.patterns {
                let source: Wtf16 = (&**source).into();
                let arena = AllArenas::default();
                let ast = generate_ast(&source, options, &arena);

                fn get_node_type(node: &Node) -> &'static str {
                    match node {
                        Node::Alternative(_) => "Alternative",
                        Node::CapturingGroup(_) => "CapturingGroup",
                        Node::CharacterClass(_) => "CharacterClass",
                        Node::CharacterClassRange(_) => "CharacterClassRange",
                        Node::ClassIntersection(_) => "ClassIntersection",
                        Node::ClassStringDisjunction(_) => "ClassStringDisjunction",
                        Node::ClassSubtraction(_) => "ClassSubtraction",
                        Node::ExpressionCharacterClass(_) => "ExpressionCharacterClass",
                        Node::Group(_) => "Group",
                        Node::Assertion(_) => "Assertion",
                        Node::Pattern(_) => "Pattern",
                        Node::Quantifier(_) => "Quantifier",
                        Node::RegExpLiteral(_) => "RegExpLiteral",
                        Node::StringAlternative(_) => "StringAlternative",
                        Node::Backreference(_) => "Backreference",
                        Node::Character(_) => "Character",
                        Node::CharacterSet(_) => "CharacterSet",
                        Node::Flags(_) => "Flags",
                    }
                }

                struct HistoryRecorder<'a> {
                    arena: &'a AllArenas,
                    history: RefCell<Vec<Wtf16>>,
                }

                impl<'a> HistoryRecorder<'a> {
                    pub fn new(arena: &'a AllArenas) -> Self {
                        Self {
                            arena,
                            history: Default::default(),
                        }
                    }

                    fn push_event(&self, node: Id<Node>, event_type: &str) {
                        let mut event: Wtf16 = (&*format!("{event_type}:")).into();
                        let node_ref = self.arena.node(node);
                        event.extend(
                            Wtf16::from(&*format!("{}:", get_node_type(&node_ref)))
                                .iter()
                                .copied(),
                        );
                        event.extend(node_ref.raw().into_iter().copied());
                        self.history.borrow_mut().push(event);
                    }

                    fn enter(&self, node: Id<Node>) {
                        self.push_event(node, "enter");
                    }

                    fn leave(&self, node: Id<Node>) {
                        self.push_event(node, "leave");
                    }
                }

                impl<'a> visitor::Handlers for HistoryRecorder<'a> {
                    fn on_alternative_enter(&self, node: Id<Node /*Alternative*/>) {
                        self.enter(node);
                    }

                    fn on_alternative_leave(&self, node: Id<Node /*Alternative*/>) {
                        self.leave(node);
                    }

                    fn on_assertion_enter(&self, node: Id<Node /*Assertion*/>) {
                        self.enter(node);
                    }

                    fn on_assertion_leave(&self, node: Id<Node /*Assertion*/>) {
                        self.leave(node);
                    }

                    fn on_backreference_enter(&self, node: Id<Node /*Backreference*/>) {
                        self.enter(node);
                    }

                    fn on_backreference_leave(&self, node: Id<Node /*Backreference*/>) {
                        self.leave(node);
                    }

                    fn on_capturing_group_enter(&self, node: Id<Node /*CapturingGroup*/>) {
                        self.enter(node);
                    }

                    fn on_capturing_group_leave(&self, node: Id<Node /*CapturingGroup*/>) {
                        self.leave(node);
                    }

                    fn on_character_enter(&self, node: Id<Node /*Character*/>) {
                        self.enter(node);
                    }

                    fn on_character_leave(&self, node: Id<Node /*Character*/>) {
                        self.leave(node);
                    }

                    fn on_character_class_enter(&self, node: Id<Node /*CharacterClass*/>) {
                        self.enter(node);
                    }

                    fn on_character_class_leave(&self, node: Id<Node /*CharacterClass*/>) {
                        self.leave(node);
                    }

                    fn on_character_class_range_enter(
                        &self,
                        node: Id<Node /*CharacterClassRange*/>,
                    ) {
                        self.enter(node);
                    }

                    fn on_character_class_range_leave(
                        &self,
                        node: Id<Node /*CharacterClassRange*/>,
                    ) {
                        self.leave(node);
                    }

                    fn on_character_set_enter(&self, node: Id<Node /*CharacterSet*/>) {
                        self.enter(node);
                    }

                    fn on_character_set_leave(&self, node: Id<Node /*CharacterSet*/>) {
                        self.leave(node);
                    }

                    fn on_class_intersection_enter(&self, node: Id<Node /*ClassIntersection*/>) {
                        self.enter(node);
                    }

                    fn on_class_intersection_leave(&self, node: Id<Node /*ClassIntersection*/>) {
                        self.leave(node);
                    }

                    fn on_class_string_disjunction_enter(
                        &self,
                        node: Id<Node /*ClassStringDisjunction*/>,
                    ) {
                        self.enter(node);
                    }

                    fn on_class_string_disjunction_leave(
                        &self,
                        node: Id<Node /*ClassStringDisjunction*/>,
                    ) {
                        self.leave(node);
                    }

                    fn on_class_subtraction_enter(&self, node: Id<Node /*ClassSubtraction*/>) {
                        self.enter(node);
                    }

                    fn on_class_subtraction_leave(&self, node: Id<Node /*ClassSubtraction*/>) {
                        self.leave(node);
                    }

                    fn on_expression_character_class_enter(
                        &self,
                        node: Id<Node /*ExpressionCharacterClass*/>,
                    ) {
                        self.enter(node);
                    }

                    fn on_expression_character_class_leave(
                        &self,
                        node: Id<Node /*ExpressionCharacterClass*/>,
                    ) {
                        self.leave(node);
                    }

                    fn on_flags_enter(&self, node: Id<Node /*Flags*/>) {
                        self.enter(node);
                    }

                    fn on_flags_leave(&self, node: Id<Node /*Flags*/>) {
                        self.leave(node);
                    }

                    fn on_group_enter(&self, node: Id<Node /*Group*/>) {
                        self.enter(node);
                    }

                    fn on_group_leave(&self, node: Id<Node /*Group*/>) {
                        self.leave(node);
                    }

                    fn on_pattern_enter(&self, node: Id<Node /*Pattern*/>) {
                        self.enter(node);
                    }

                    fn on_pattern_leave(&self, node: Id<Node /*Pattern*/>) {
                        self.leave(node);
                    }

                    fn on_quantifier_enter(&self, node: Id<Node /*Quantifier*/>) {
                        self.enter(node);
                    }

                    fn on_quantifier_leave(&self, node: Id<Node /*Quantifier*/>) {
                        self.leave(node);
                    }

                    fn on_reg_exp_literal_enter(&self, node: Id<Node /*RegExpLiteral*/>) {
                        self.enter(node);
                    }

                    fn on_reg_exp_literal_leave(&self, node: Id<Node /*RegExpLiteral*/>) {
                        self.leave(node);
                    }

                    fn on_string_alternative_enter(&self, node: Id<Node /*StringAlternative*/>) {
                        self.enter(node);
                    }

                    fn on_string_alternative_leave(&self, node: Id<Node /*StringAlternative*/>) {
                        self.leave(node);
                    }
                }

                let history = HistoryRecorder::new(&arena);
                visit_reg_exp_ast(ast, &history, &arena);

                assert_that!(&*history.history.borrow()).is_equal_to(expected);
            }
        }
    }
}
