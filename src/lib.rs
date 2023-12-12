#![allow(clippy::into_iter_on_ref)]

mod arena;
mod ast;
mod ecma_versions;
mod parser;
mod reader;
mod regexp_syntax_error;
#[cfg(test)]
mod test;
mod unicode;
mod validator;
mod wtf16;

use arena::AllArenas;
use ast::Node;
pub(crate) use ecma_versions::EcmaVersion;
use id_arena::Id;
use parser::RegExpParser;
pub use reader::{CodePoint, Reader};
pub use regexp_syntax_error::RegExpSyntaxError;
pub use validator::RegExpValidator;
use wtf16::Wtf16;

pub type Result<T> = std::result::Result<T, RegExpSyntaxError>;

pub fn parse_reg_exp_literal(
    source: &[u16],
    options: Option<parser::Options>,
    arena: &AllArenas,
) -> Result<Id<Node> /*AST.RegExpLiteral*/> {
    RegExpParser::new(arena, options).parse_literal(source, None, None)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use itertools::Itertools;
    use regex::Captures;
    use speculoos::prelude::*;
    use squalid::regex;

    use super::*;

    use crate::{
        ast::{resolve_location, to_node_unresolved, NodeUnresolved},
        test::fixtures::parser::literal::{AstOrError, FIXTURES_DATA},
        unicode::{
            is_line_terminator, ASTERISK, LEFT_SQUARE_BRACKET, REVERSE_SOLIDUS,
            RIGHT_SQUARE_BRACKET, SOLIDUS,
        },
        validator::{bmp_char_to_utf_16, ValidatePatternFlags},
    };

    fn generate_ast(source: &[u16], options: parser::Options, arena: &AllArenas) -> NodeUnresolved {
        let ast = parse_reg_exp_literal(&source, Some(options), &arena).unwrap();
        let mut path: Vec<String> = Default::default();
        let mut path_map: HashMap<Id<Node>, String> = Default::default();
        resolve_location(&arena, ast, &mut path, &mut path_map);
        to_node_unresolved(ast, &arena, &path_map)
    }

    #[test]
    fn test_parse_reg_exp_literal_fixtures() {
        for (filename, fixture) in &*FIXTURES_DATA {
            println!("filename: {filename:#?}");
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
                println!("source: {source:#?}");
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
                            let expected_message = regex!(r#"/([a-z]+?):"#).replace(
                                &expected.message,
                                |captures: &Captures| {
                                    format!(
                                        "/{}:",
                                        regex!(r#"[^uv]"#).replace_all(&captures[1], "")
                                    )
                                }
                            ).into_owned();
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

        if chars[0] != vec![bmp_char_to_utf_16('/')].into() {
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
}
