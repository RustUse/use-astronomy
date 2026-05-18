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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrbitTextError {
    EmptyName,
}

impl fmt::Display for OrbitTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("orbit name cannot be empty"),
        }
    }
}

impl Error for OrbitTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrbitName(String);

impl OrbitName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, OrbitTextError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            Err(OrbitTextError::EmptyName)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrbitName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OrbitName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrbitName {
    type Err = OrbitTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrbitKind {
    Circular,
    Elliptical,
    Parabolic,
    Hyperbolic,
    Geocentric,
    Heliocentric,
    Areocentric,
    Selenocentric,
    Barycentric,
    Unknown,
    Custom(String),
}

impl fmt::Display for OrbitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Circular => formatter.write_str("circular"),
            Self::Elliptical => formatter.write_str("elliptical"),
            Self::Parabolic => formatter.write_str("parabolic"),
            Self::Hyperbolic => formatter.write_str("hyperbolic"),
            Self::Geocentric => formatter.write_str("geocentric"),
            Self::Heliocentric => formatter.write_str("heliocentric"),
            Self::Areocentric => formatter.write_str("areocentric"),
            Self::Selenocentric => formatter.write_str("selenocentric"),
            Self::Barycentric => formatter.write_str("barycentric"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrbitKindParseError {
    Empty,
}

impl fmt::Display for OrbitKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("orbit kind cannot be empty"),
        }
    }
}

impl Error for OrbitKindParseError {}

impl FromStr for OrbitKind {
    type Err = OrbitKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrbitKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "circular" => Ok(Self::Circular),
            "elliptical" => Ok(Self::Elliptical),
            "parabolic" => Ok(Self::Parabolic),
            "hyperbolic" => Ok(Self::Hyperbolic),
            "geocentric" => Ok(Self::Geocentric),
            "heliocentric" => Ok(Self::Heliocentric),
            "areocentric" => Ok(Self::Areocentric),
            "selenocentric" => Ok(Self::Selenocentric),
            "barycentric" => Ok(Self::Barycentric),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrbitDirection {
    Prograde,
    Retrograde,
    Polar,
    Unknown,
    Custom(String),
}

impl fmt::Display for OrbitDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prograde => formatter.write_str("prograde"),
            Self::Retrograde => formatter.write_str("retrograde"),
            Self::Polar => formatter.write_str("polar"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrbitDirectionParseError {
    Empty,
}

impl fmt::Display for OrbitDirectionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("orbit direction cannot be empty"),
        }
    }
}

impl Error for OrbitDirectionParseError {}

impl FromStr for OrbitDirection {
    type Err = OrbitDirectionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrbitDirectionParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "prograde" => Ok(Self::Prograde),
            "retrograde" => Ok(Self::Retrograde),
            "polar" => Ok(Self::Polar),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrbitState {
    Bound,
    Escape,
    Transfer,
    Decaying,
    Unknown,
    Custom(String),
}

impl fmt::Display for OrbitState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bound => formatter.write_str("bound"),
            Self::Escape => formatter.write_str("escape"),
            Self::Transfer => formatter.write_str("transfer"),
            Self::Decaying => formatter.write_str("decaying"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrbitStateParseError {
    Empty,
}

impl fmt::Display for OrbitStateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("orbit state cannot be empty"),
        }
    }
}

impl Error for OrbitStateParseError {}

impl FromStr for OrbitState {
    type Err = OrbitStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrbitStateParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "bound" => Ok(Self::Bound),
            "escape" => Ok(Self::Escape),
            "transfer" => Ok(Self::Transfer),
            "decaying" => Ok(Self::Decaying),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OrbitDirection, OrbitKind, OrbitName, OrbitState, OrbitTextError};

    #[test]
    fn valid_orbit_name() {
        let name = OrbitName::new("Earth heliocentric orbit").unwrap();

        assert_eq!(name.as_str(), "Earth heliocentric orbit");
    }

    #[test]
    fn empty_orbit_name_rejected() {
        assert_eq!(OrbitName::new("  "), Err(OrbitTextError::EmptyName));
    }

    #[test]
    fn orbit_kind_display_and_parse() {
        assert_eq!(OrbitKind::Heliocentric.to_string(), "heliocentric");
        assert_eq!(
            "barycentric".parse::<OrbitKind>().unwrap(),
            OrbitKind::Barycentric
        );
    }

    #[test]
    fn orbit_direction_display_and_parse() {
        assert_eq!(OrbitDirection::Prograde.to_string(), "prograde");
        assert_eq!(
            "polar".parse::<OrbitDirection>().unwrap(),
            OrbitDirection::Polar
        );
    }

    #[test]
    fn orbit_state_display_and_parse() {
        assert_eq!(OrbitState::Bound.to_string(), "bound");
        assert_eq!(
            "transfer".parse::<OrbitState>().unwrap(),
            OrbitState::Transfer
        );
    }

    #[test]
    fn custom_orbit_kind() {
        assert_eq!(
            "graveyard".parse::<OrbitKind>().unwrap(),
            OrbitKind::Custom("graveyard".to_string())
        );
    }
}
