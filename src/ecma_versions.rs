#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcmaVersion {
    _5,
    _2015,
    _2016,
    _2017,
    _2018,
    _2019,
    _2020,
    _2021,
    _2022,
    _2023,
    _2024,
}

impl TryFrom<u32> for EcmaVersion {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            5 => EcmaVersion::_5,
            2015 => EcmaVersion::_2015,
            2016 => EcmaVersion::_2016,
            2017 => EcmaVersion::_2017,
            2018 => EcmaVersion::_2018,
            2019 => EcmaVersion::_2019,
            2020 => EcmaVersion::_2020,
            2021 => EcmaVersion::_2021,
            2022 => EcmaVersion::_2022,
            2023 => EcmaVersion::_2023,
            2024 => EcmaVersion::_2024,
            _ => return Err(format!("'{}' is not a valid ECMA version", value)),
        })
    }
}

pub const LATEST_ECMA_VERSION: EcmaVersion = EcmaVersion::_2024;
