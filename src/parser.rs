use crate::ecma_versions::EcmaVersion;

#[derive(Copy, Clone, Default)]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
}
