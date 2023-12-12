use std::{mem, ops, string::FromUtf16Error};

use serde::{Deserialize, Deserializer};
use serde_bytes::ByteBuf;
use wtf8::Wtf8;

use crate::{unicode::{is_lead_surrogate, is_trail_surrogate}, CodePoint};

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
        D: Deserializer<'de>,
    {
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

impl ops::DerefMut for Wtf16 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl From<CodePoint> for Wtf16 {
    fn from(value: CodePoint) -> Self {
        Self(
            if value > 0xffff {
                let mut buffer: Vec<u16> = Vec::with_capacity(2);
                char::try_from(value).unwrap().encode_utf16(&mut buffer);
                buffer
            } else {
                vec![u16::try_from(value).unwrap()]
            }
            .into(),
        )
    }
}

impl TryFrom<&Wtf16> for String {
    type Error = FromUtf16Error;

    fn try_from(value: &Wtf16) -> Result<Self, Self::Error> {
        Self::from_utf16(value)
    }
}

pub struct SplitCodePoints<'a> {
    original: &'a Wtf16,
    next_index: usize,
}

impl<'a> SplitCodePoints<'a> {
    pub fn new(original: &'a Wtf16) -> Self {
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
        if is_lead_surrogate(next_unit.into()) {
            if let Some(second_unit) = self
                .original
                .get(self.next_index + 1)
                .copied()
                .filter(|&second_unit| is_trail_surrogate(second_unit.into()))
            {
                self.next_index += 2;
                return Some(vec![next_unit, second_unit].into());
            }
        }
        self.next_index += 1;
        Some(vec![next_unit].into())
    }
}

pub fn is_surrogate_code_point(value: u16) -> bool {
    (0xd800..=0xdfff).contains(&value)
}

pub fn get_single_surrogate_pair_code_point(values: &[u16]) -> CodePoint {
    let mut iterator = char::decode_utf16(values.into_iter().copied());
    let first_char = iterator
        .next()
        .expect("Should've gotten at least one char")
        .expect("Expected valid surrogate pair");
    assert!(iterator.next().is_none(), "Expected only one char");
    let first_code_point: CodePoint = first_char.into();
    assert!(first_code_point > 0xffff);
    first_code_point
}
