use std::{fs, path::PathBuf};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{parser, wtf16::Wtf16};

pub type FixtureData = IndexMap<PathBuf, FixtureDataValue>;

#[derive(Deserialize)]
pub struct FixtureDataValue {
    pub options: parser::Options,
    pub patterns: IndexMap<String, Vec<Wtf16>>,
}

static FIXTURES_ROOT: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(file!()).parent().unwrap().to_owned());

pub static FIXTURES_DATA: Lazy<FixtureData> = Lazy::new(|| {
    fs::read_dir(&*FIXTURES_ROOT)
        .unwrap()
        .map(Result::unwrap)
        .filter_map(|dirent| {
            (dirent.path().extension().is_some() && dirent.path().extension().unwrap() == "json")
                .then(|| dirent.path())
        })
        .map(|filename| {
            (
                filename.clone(),
                serde_json::from_str::<FixtureDataValue>(&fs::read_to_string(filename).unwrap())
                    .unwrap(),
            )
        })
        .collect()
});
