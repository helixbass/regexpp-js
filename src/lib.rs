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

pub fn parse_reg_exp_literal<'a>(
    source: &str,
    options: Option<parser::Options>,
    arena: &AllArenas<'a>,
) -> Id<Node<'a>> /*AST.RegExpLiteral*/ {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    use crate::test::fixtures::parser::literal::{AstOrError, FIXTURES_DATA};

    #[test]
    fn test_parse_reg_exp_literal_fixtures() {
        for (filename, fixture) in &*FIXTURES_DATA {
            let options = fixture.options;
            if filename.to_str().unwrap().contains("-valid") {
                for (source, result) in &fixture.patterns {
                    assert_that!(&matches!(result, AstOrError::Ast(_),)).is_true();
                }
            }
        }
    }
}
