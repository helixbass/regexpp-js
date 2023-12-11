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

use arena::AllArenas;
use ast::Node;
pub(crate) use ecma_versions::EcmaVersion;
use id_arena::Id;
pub use reader::{CodePoint, Reader};
pub use regexp_syntax_error::RegExpSyntaxError;
pub use validator::RegExpValidator;

pub type Result<T> = std::result::Result<T, RegExpSyntaxError>;

pub fn parse_reg_exp_literal<'a>(
    source: &str,
    options: Option<parser::Options>,
    arena: &mut AllArenas<'a>,
) -> Result<Id<Node<'a>> /*AST.RegExpLiteral*/> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use speculoos::prelude::*;

    use super::*;

    use crate::{test::fixtures::parser::literal::{AstOrError, FIXTURES_DATA}, ast::{NodeUnresolved, to_node_unresolved, resolve_location}};

    fn generate_ast<'a>(source: &'a str, options: parser::Options) -> NodeUnresolved {
        let mut arena: AllArenas<'a> = Default::default();
        let ast = parse_reg_exp_literal(source, Some(options), &mut arena).unwrap();
        let mut path: Vec<String> = Default::default();
        let mut path_map: HashMap<Id<Node<'a>>, String> = Default::default();
        resolve_location(
            &arena,
            ast,
            &mut path,
            &mut path_map,
        );
        to_node_unresolved(ast, &arena, &path_map)
    }

    #[test]
    fn test_parse_reg_exp_literal_fixtures() {
        for (filename, fixture) in &*FIXTURES_DATA {
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
                match result {
                    AstOrError::Ast(expected) => {
                        let actual = generate_ast(source, options);
                        assert_that!(&actual).is_equal_to(expected);
                    }
                    AstOrError::Error(result) => {
                        unimplemented!()
                    }
                }
            }
        }
    }
}
