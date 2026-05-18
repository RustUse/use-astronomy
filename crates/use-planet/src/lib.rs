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
    error: PlanetTextError,
) -> Result<String, PlanetTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanetTextError {
    EmptyName,
    EmptySystemName,
}

impl fmt::Display for PlanetTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("planet name cannot be empty"),
            Self::EmptySystemName => formatter.write_str("planetary system name cannot be empty"),
        }
    }
}

impl Error for PlanetTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlanetName(String);

impl PlanetName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlanetTextError> {
        non_empty_text(value, PlanetTextError::EmptyName).map(Self)
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

impl AsRef<str> for PlanetName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlanetName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlanetName {
    type Err = PlanetTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlanetarySystemName(String);

impl PlanetarySystemName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlanetTextError> {
        non_empty_text(value, PlanetTextError::EmptySystemName).map(Self)
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

impl AsRef<str> for PlanetarySystemName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlanetarySystemName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlanetarySystemName {
    type Err = PlanetTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlanetKind {
    Terrestrial,
    GasGiant,
    IceGiant,
    DwarfPlanet,
    Exoplanet,
    RoguePlanet,
    Unknown,
    Custom(String),
}

impl fmt::Display for PlanetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terrestrial => formatter.write_str("terrestrial"),
            Self::GasGiant => formatter.write_str("gas-giant"),
            Self::IceGiant => formatter.write_str("ice-giant"),
            Self::DwarfPlanet => formatter.write_str("dwarf-planet"),
            Self::Exoplanet => formatter.write_str("exoplanet"),
            Self::RoguePlanet => formatter.write_str("rogue-planet"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanetKindParseError {
    Empty,
}

impl fmt::Display for PlanetKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("planet kind cannot be empty"),
        }
    }
}

impl Error for PlanetKindParseError {}

impl FromStr for PlanetKind {
    type Err = PlanetKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PlanetKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "terrestrial" => Ok(Self::Terrestrial),
            "gas-giant" | "gasgiant" => Ok(Self::GasGiant),
            "ice-giant" | "icegiant" => Ok(Self::IceGiant),
            "dwarf-planet" | "dwarfplanet" => Ok(Self::DwarfPlanet),
            "exoplanet" => Ok(Self::Exoplanet),
            "rogue-planet" | "rogueplanet" => Ok(Self::RoguePlanet),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlanetStatus {
    Confirmed,
    Candidate,
    Provisional,
    Disputed,
    Unknown,
    Custom(String),
}

impl fmt::Display for PlanetStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed => formatter.write_str("confirmed"),
            Self::Candidate => formatter.write_str("candidate"),
            Self::Provisional => formatter.write_str("provisional"),
            Self::Disputed => formatter.write_str("disputed"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanetStatusParseError {
    Empty,
}

impl fmt::Display for PlanetStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("planet status cannot be empty"),
        }
    }
}

impl Error for PlanetStatusParseError {}

impl FromStr for PlanetStatus {
    type Err = PlanetStatusParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PlanetStatusParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "confirmed" => Ok(Self::Confirmed),
            "candidate" => Ok(Self::Candidate),
            "provisional" => Ok(Self::Provisional),
            "disputed" => Ok(Self::Disputed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PlanetKind, PlanetName, PlanetStatus, PlanetTextError, PlanetarySystemName};

    #[test]
    fn valid_planet_name() {
        let name = PlanetName::new("Mercury").unwrap();

        assert_eq!(name.as_str(), "Mercury");
    }

    #[test]
    fn empty_planet_name_rejected() {
        assert_eq!(PlanetName::new("  "), Err(PlanetTextError::EmptyName));
    }

    #[test]
    fn planet_kind_display_and_parse() {
        assert_eq!(PlanetKind::GasGiant.to_string(), "gas-giant");
        assert_eq!(
            "ice giant".parse::<PlanetKind>().unwrap(),
            PlanetKind::IceGiant
        );
    }

    #[test]
    fn planet_status_display_and_parse() {
        assert_eq!(PlanetStatus::Confirmed.to_string(), "confirmed");
        assert_eq!(
            "disputed".parse::<PlanetStatus>().unwrap(),
            PlanetStatus::Disputed
        );
    }

    #[test]
    fn custom_planet_kind() {
        assert_eq!(
            "super-puff".parse::<PlanetKind>().unwrap(),
            PlanetKind::Custom("super-puff".to_string())
        );
    }

    #[test]
    fn planetary_system_name_construction() {
        let system = PlanetarySystemName::new("TRAPPIST-1").unwrap();

        assert_eq!(system.as_str(), "TRAPPIST-1");
    }
}
