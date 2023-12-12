use std::{ops, mem};

use serde::{Deserialize, Deserializer};
use serde_bytes::ByteBuf;
use wtf8::Wtf8;

use crate::reader::is_surrogate_code_point;


#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Wtf16(Vec<u16>);

impl Wtf16 {
    pub fn new(code_units: Vec<u16>) -> Self {
        Self(code_units)
    }

    pub fn split_code_points(&self) -> SplitCodePoints {
        SplitCodePoints::new(self)
    }
}

impl<'de> Deserialize<'de> for Wtf16 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let bytes = ByteBuf::deserialize(deserializer)?.into_vec();
        let as_str: &str = unsafe { mem::transmute(&*bytes) };
        Ok(as_str.into())
    }
}

impl ops::Deref for Wtf16 {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u16]> for Wtf16 {
    fn from(value: &[u16]) -> Self {
        Self(value.to_owned())
    }
}

impl From<Vec<u16>> for Wtf16 {
    fn from(value: Vec<u16>) -> Self {
        Self(value)
    }
}

impl From<&str> for Wtf16 {
    fn from(value: &str) -> Self {
        let wtf8 = Wtf8::from_str(value);
        Self::new(wtf8.to_ill_formed_utf16().collect())
    }
}

pub struct SplitCodePoints<'a> {
    original: &'a Wtf16,
    next_index: usize,
}

impl<'a> SplitCodePoints<'a> {
    pub fn new(
        original: &'a Wtf16,
    ) -> Self {
        Self {
            original,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for SplitCodePoints<'a> {
    type Item = Wtf16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.original.len() {
            return None;
        }
        let next_unit = self.original[self.next_index];
        if !is_surrogate_code_point(next_unit) {
            return Some(vec![next_unit].into());
        }
        unimplemented!()
    }
}