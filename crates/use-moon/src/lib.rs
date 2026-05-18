#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|character| match character {
            '_' | ' ' => '-',
            other => other.to_ascii_lowercase(),
        })
        .collect()
}

fn non_empty_text(value: impl AsRef<str>, error: MoonTextError) -> Result<String, MoonTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoonTextError {
    EmptyName,
    EmptyParentIdentifier,
    EmptySatelliteIdentifier,
}

impl fmt::Display for MoonTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("moon name cannot be empty"),
            Self::EmptyParentIdentifier => formatter.write_str("parent identifier cannot be empty"),
            Self::EmptySatelliteIdentifier => {
                formatter.write_str("satellite identifier cannot be empty")
            },
        }
    }
}

impl Error for MoonTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoonName(String);

impl MoonName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, MoonTextError> {
        non_empty_text(value, MoonTextError::EmptyName).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MoonName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for MoonName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MoonName {
    type Err = MoonTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MoonKind {
    Regular,
    Irregular,
    Captured,
    Shepherd,
    Trojan,
    Unknown,
    Custom(String),
}

impl fmt::Display for MoonKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => formatter.write_str("regular"),
            Self::Irregular => formatter.write_str("irregular"),
            Self::Captured => formatter.write_str("captured"),
            Self::Shepherd => formatter.write_str("shepherd"),
            Self::Trojan => formatter.write_str("trojan"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoonKindParseError {
    Empty,
}

impl fmt::Display for MoonKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("moon kind cannot be empty"),
        }
    }
}

impl Error for MoonKindParseError {}

impl FromStr for MoonKind {
    type Err = MoonKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(MoonKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "regular" => Ok(Self::Regular),
            "irregular" => Ok(Self::Irregular),
            "captured" => Ok(Self::Captured),
            "shepherd" => Ok(Self::Shepherd),
            "trojan" => Ok(Self::Trojan),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SatelliteRelation {
    parent_identifier: String,
    satellite_identifier: String,
}

impl SatelliteRelation {
    pub fn new(
        parent_identifier: impl AsRef<str>,
        satellite_identifier: impl AsRef<str>,
    ) -> Result<Self, MoonTextError> {
        Ok(Self {
            parent_identifier: non_empty_text(
                parent_identifier,
                MoonTextError::EmptyParentIdentifier,
            )?,
            satellite_identifier: non_empty_text(
                satellite_identifier,
                MoonTextError::EmptySatelliteIdentifier,
            )?,
        })
    }

    #[must_use]
    pub fn parent_identifier(&self) -> &str {
        &self.parent_identifier
    }

    #[must_use]
    pub fn satellite_identifier(&self) -> &str {
        &self.satellite_identifier
    }
}

#[cfg(test)]
mod tests {
    use super::{MoonKind, MoonName, MoonTextError, SatelliteRelation};

    #[test]
    fn valid_moon_name() {
        let name = MoonName::new("Europa").unwrap();

        assert_eq!(name.as_str(), "Europa");
    }

    #[test]
    fn empty_moon_name_rejected() {
        assert_eq!(MoonName::new("   "), Err(MoonTextError::EmptyName));
    }

    #[test]
    fn moon_kind_display_and_parse() {
        assert_eq!(MoonKind::Regular.to_string(), "regular");
        assert_eq!("captured".parse::<MoonKind>().unwrap(), MoonKind::Captured);
    }

    #[test]
    fn custom_moon_kind() {
        assert_eq!(
            "resonant".parse::<MoonKind>().unwrap(),
            MoonKind::Custom("resonant".to_string())
        );
    }

    #[test]
    fn satellite_relation_construction() {
        let relation = SatelliteRelation::new("Jupiter", "Europa").unwrap();

        assert_eq!(relation.parent_identifier(), "Jupiter");
        assert_eq!(relation.satellite_identifier(), "Europa");
    }
}
