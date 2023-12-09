#![cfg(test)]

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use itertools::{Either, Itertools};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};

use crate::{ecma_versions::EcmaVersion, RegExpSyntaxError};

pub type FixtureData = HashMap<PathBuf, FixtureDataValue>;

#[derive(Deserialize)]
pub struct FixtureDataValue {
    pub options: FixtureDataOptions,
    pub patterns: HashMap<String, AstOrError>,
}

fn deserialize_ecma_version<'de, D>(deserializer: D) -> Result<EcmaVersion, D::Error>
    where D: Deserializer<'de> {
    let ecma_version = u32::deserialize(deserializer)?;
    EcmaVersion::try_from(ecma_version).map_err(serde::de::Error::custom)
}

#[derive(Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureDataOptions {
    pub strict: bool,
    #[serde(deserialize_with = "deserialize_ecma_version")]
    pub ecma_version: EcmaVersion,
}

#[derive(Deserialize)]
pub enum AstOrError {
    Ast(serde_json::Value),
    Error(RegExpSyntaxError),
}

static FIXTURES_ROOT: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(file!()).parent().unwrap().to_owned());

pub static FIXTURES_DATA: Lazy<FixtureData> = Lazy::new(|| {
    extract_fixture_files(&FIXTURES_ROOT)
        .into_iter()
        .map(|filename| {
            println!("deserializing {filename:#?}");
            (
                filename.clone(),
                serde_json::from_str(&fs::read_to_string(filename).unwrap()).unwrap(),
            )
        })
        .collect()
});

fn extract_fixture_files(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(Result::unwrap)
        .flat_map(|dirent| {
            if dirent.file_type().unwrap().is_dir() {
                Either::Left(extract_fixture_files(&dirent.path()).into_iter())
            } else {
                Either::Right(
                    (dirent.path().extension().is_some()
                        && dirent.path().extension().unwrap() == "json")
                        .then(|| vec![dirent.path()])
                        .unwrap_or_default()
                        .into_iter(),
                )
            }
        })
        .collect_vec()
}
