use std::borrow::Cow;

use serde::Deserialize;
use squalid::NonEmpty;

use crate::validator::{RegExpValidatorSourceContext, RegExpValidatorSourceContextKind};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RegExpSyntaxError {
    pub message: String,
    pub index: usize,
}

#[derive(Copy, Clone)]
pub struct Flags {
    pub unicode: bool,
    pub unicode_sets: bool,
}

pub fn new_reg_exp_syntax_error(
    src_ctx: &RegExpValidatorSourceContext,
    flags: Flags,
    index: usize,
    message: &str
) -> RegExpSyntaxError {
    let mut source: Cow<'_, str> = "".into();
    match src_ctx.kind {
        RegExpValidatorSourceContextKind::Literal => {
            let literal = &src_ctx.source[src_ctx.start..src_ctx.end];
            if let Some(literal) = literal.non_empty() {
                source = format!(": {literal}").into();
            }
        }
        RegExpValidatorSourceContextKind::Pattern => {
            let pattern = &src_ctx.source[src_ctx.start..src_ctx.end];
            let flags_text = format!("{}{}", if flags.unicode {
                "u"
            } else {
                ""
            }, if flags.unicode_sets {
                "v"
            } else {
                ""
            });
            source = format!(": /{pattern}/{flags_text}").into();
        }
        _ => ()
    }

    RegExpSyntaxError {
        message: format!("Invalid regular expression{source}: {message}"),
        index,
    }
}
