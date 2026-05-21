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

fn non_empty_text(
    value: impl AsRef<str>,
    error: CelestialBodyTextError,
) -> Result<String, CelestialBodyTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CelestialBodyTextError {
    EmptyName,
    EmptyId,
}

impl fmt::Display for CelestialBodyTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("celestial body name cannot be empty"),
            Self::EmptyId => formatter.write_str("celestial body identifier cannot be empty"),
        }
    }
}

impl Error for CelestialBodyTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CelestialBodyName(String);

impl CelestialBodyName {
    /// Creates a celestial body name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CelestialBodyTextError::EmptyName`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CelestialBodyTextError> {
        non_empty_text(value, CelestialBodyTextError::EmptyName).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for CelestialBodyName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CelestialBodyName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CelestialBodyName {
    type Err = CelestialBodyTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CelestialBodyId(String);

impl CelestialBodyId {
    /// Creates a celestial body identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CelestialBodyTextError::EmptyId`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CelestialBodyTextError> {
        non_empty_text(value, CelestialBodyTextError::EmptyId).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for CelestialBodyId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CelestialBodyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CelestialBodyId {
    type Err = CelestialBodyTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CelestialBodyKind {
    Star,
    Planet,
    DwarfPlanet,
    Moon,
    Asteroid,
    Comet,
    Meteoroid,
    Nebula,
    Galaxy,
    BlackHole,
    StarCluster,
    Unknown,
    Custom(String),
}

impl fmt::Display for CelestialBodyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Star => formatter.write_str("star"),
            Self::Planet => formatter.write_str("planet"),
            Self::DwarfPlanet => formatter.write_str("dwarf-planet"),
            Self::Moon => formatter.write_str("moon"),
            Self::Asteroid => formatter.write_str("asteroid"),
            Self::Comet => formatter.write_str("comet"),
            Self::Meteoroid => formatter.write_str("meteoroid"),
            Self::Nebula => formatter.write_str("nebula"),
            Self::Galaxy => formatter.write_str("galaxy"),
            Self::BlackHole => formatter.write_str("black-hole"),
            Self::StarCluster => formatter.write_str("star-cluster"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CelestialBodyKindParseError {
    Empty,
}

impl fmt::Display for CelestialBodyKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("celestial body kind cannot be empty"),
        }
    }
}

impl Error for CelestialBodyKindParseError {}

impl FromStr for CelestialBodyKind {
    type Err = CelestialBodyKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CelestialBodyKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "star" => Ok(Self::Star),
            "planet" => Ok(Self::Planet),
            "dwarf-planet" | "dwarfplanet" => Ok(Self::DwarfPlanet),
            "moon" => Ok(Self::Moon),
            "asteroid" => Ok(Self::Asteroid),
            "comet" => Ok(Self::Comet),
            "meteoroid" => Ok(Self::Meteoroid),
            "nebula" => Ok(Self::Nebula),
            "galaxy" => Ok(Self::Galaxy),
            "black-hole" | "blackhole" => Ok(Self::BlackHole),
            "star-cluster" | "starcluster" => Ok(Self::StarCluster),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CelestialBodyStatus {
    Confirmed,
    Candidate,
    Provisional,
    Retired,
    Unknown,
    Custom(String),
}

impl fmt::Display for CelestialBodyStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed => formatter.write_str("confirmed"),
            Self::Candidate => formatter.write_str("candidate"),
            Self::Provisional => formatter.write_str("provisional"),
            Self::Retired => formatter.write_str("retired"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CelestialBodyStatusParseError {
    Empty,
}

impl fmt::Display for CelestialBodyStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("celestial body status cannot be empty"),
        }
    }
}

impl Error for CelestialBodyStatusParseError {}

impl FromStr for CelestialBodyStatus {
    type Err = CelestialBodyStatusParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CelestialBodyStatusParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "confirmed" => Ok(Self::Confirmed),
            "candidate" => Ok(Self::Candidate),
            "provisional" => Ok(Self::Provisional),
            "retired" => Ok(Self::Retired),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CelestialBodyId, CelestialBodyKind, CelestialBodyName, CelestialBodyStatus,
        CelestialBodyTextError,
    };

    #[test]
    fn valid_celestial_body_name() {
        let name = CelestialBodyName::new("Alpha Centauri A").unwrap();

        assert_eq!(name.as_str(), "Alpha Centauri A");
        assert_eq!(name.to_string(), "Alpha Centauri A");
    }

    #[test]
    fn empty_celestial_body_name_rejected() {
        assert_eq!(
            CelestialBodyName::new("   "),
            Err(CelestialBodyTextError::EmptyName)
        );
    }

    #[test]
    fn body_kind_display_and_parse() {
        assert_eq!(CelestialBodyKind::BlackHole.to_string(), "black-hole");
        assert_eq!(
            "dwarf planet".parse::<CelestialBodyKind>().unwrap(),
            CelestialBodyKind::DwarfPlanet
        );
    }

    #[test]
    fn custom_body_kind() {
        assert_eq!(
            "proto-planetary disk".parse::<CelestialBodyKind>().unwrap(),
            CelestialBodyKind::Custom("proto-planetary disk".to_string())
        );
    }

    #[test]
    fn body_status_display_and_parse() {
        assert_eq!(CelestialBodyStatus::Confirmed.to_string(), "confirmed");
        assert_eq!(
            "provisional".parse::<CelestialBodyStatus>().unwrap(),
            CelestialBodyStatus::Provisional
        );
    }

    #[test]
    fn body_id_construction() {
        let identifier = CelestialBodyId::new("HIP 71683").unwrap();

        assert_eq!(identifier.as_str(), "HIP 71683");
    }
}
