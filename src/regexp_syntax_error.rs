use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RegExpSyntaxError {
    pub message: String,
    pub index: usize,
}
