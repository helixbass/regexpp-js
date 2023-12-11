use serde::Deserialize;

use crate::ecma_versions::EcmaVersion;

#[derive(Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    strict: Option<bool>,
    ecma_version: Option<EcmaVersion>,
}
