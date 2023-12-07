use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegExpSyntaxError {
    pub message: String,
    pub index: usize,
}
