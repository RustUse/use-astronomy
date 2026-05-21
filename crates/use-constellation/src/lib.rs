#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(
    value: impl AsRef<str>,
    error: ConstellationTextError,
) -> Result<String, ConstellationTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstellationTextError {
    EmptyName,
    EmptyAbbreviation,
    EmptyRegion,
}

impl fmt::Display for ConstellationTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("constellation name cannot be empty"),
            Self::EmptyAbbreviation => {
                formatter.write_str("constellation abbreviation cannot be empty")
            },
            Self::EmptyRegion => formatter.write_str("constellation region cannot be empty"),
        }
    }
}

impl Error for ConstellationTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstellationName(String);

impl ConstellationName {
    /// Creates a constellation name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ConstellationTextError::EmptyName`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConstellationTextError> {
        non_empty_text(value, ConstellationTextError::EmptyName).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ConstellationName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ConstellationName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ConstellationName {
    type Err = ConstellationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstellationAbbreviation(String);

impl ConstellationAbbreviation {
    /// Creates a constellation abbreviation from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ConstellationTextError::EmptyAbbreviation`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConstellationTextError> {
        non_empty_text(value, ConstellationTextError::EmptyAbbreviation).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ConstellationAbbreviation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ConstellationAbbreviation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ConstellationAbbreviation {
    type Err = ConstellationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstellationRegion(String);

impl ConstellationRegion {
    /// Creates a constellation region label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ConstellationTextError::EmptyRegion`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConstellationTextError> {
        non_empty_text(value, ConstellationTextError::EmptyRegion).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ConstellationRegion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ConstellationRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ConstellationRegion {
    type Err = ConstellationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConstellationAbbreviation, ConstellationName, ConstellationRegion, ConstellationTextError,
    };

    #[test]
    fn valid_constellation_name() {
        let name = ConstellationName::new("Lyra").unwrap();

        assert_eq!(name.as_str(), "Lyra");
    }

    #[test]
    fn empty_constellation_name_rejected() {
        assert_eq!(
            ConstellationName::new("   "),
            Err(ConstellationTextError::EmptyName)
        );
    }

    #[test]
    fn valid_abbreviation() {
        let abbreviation = ConstellationAbbreviation::new("Lyr").unwrap();

        assert_eq!(abbreviation.as_str(), "Lyr");
    }

    #[test]
    fn empty_abbreviation_rejected() {
        assert_eq!(
            ConstellationAbbreviation::new(" "),
            Err(ConstellationTextError::EmptyAbbreviation)
        );
    }

    #[test]
    fn constellation_region_construction() {
        let region = ConstellationRegion::new("northern sky").unwrap();

        assert_eq!(region.as_str(), "northern sky");
    }

    #[test]
    fn display_behavior() {
        let name = ConstellationName::new("Orion").unwrap();
        let abbreviation = ConstellationAbbreviation::new("Ori").unwrap();

        assert_eq!(name.to_string(), "Orion");
        assert_eq!(abbreviation.to_string(), "Ori");
    }
}
