#![cfg(test)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use itertools::{Either, Itertools};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{ast::NodeUnresolved, parser, RegExpSyntaxError};

pub type FixtureData = IndexMap<PathBuf, FixtureDataValue>;

#[derive(Deserialize)]
pub struct FixtureDataValue {
    pub options: parser::Options,
    pub patterns: IndexMap<String, AstOrError>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AstOrError {
    Ast(NodeUnresolved),
    Error(RegExpSyntaxError),
}

static FIXTURES_ROOT: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(file!()).parent().unwrap().to_owned());

pub static FIXTURES_DATA: Lazy<FixtureData> = Lazy::new(|| {
    extract_fixture_files(&FIXTURES_ROOT)
        .into_iter()
        .map(|filename| {
            (
                filename.clone(),
                serde_json::from_str::<FixtureDataValue>(&fs::read_to_string(filename).unwrap())
                    .unwrap(),
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
