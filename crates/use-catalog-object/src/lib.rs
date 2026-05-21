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
    error: CatalogObjectTextError,
) -> Result<String, CatalogObjectTextError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogObjectTextError {
    EmptyCatalogName,
    EmptyCatalogObjectId,
    EmptyDesignation,
}

impl fmt::Display for CatalogObjectTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCatalogName => formatter.write_str("catalog name cannot be empty"),
            Self::EmptyCatalogObjectId => {
                formatter.write_str("catalog object identifier cannot be empty")
            },
            Self::EmptyDesignation => formatter.write_str("catalog designation cannot be empty"),
        }
    }
}

impl Error for CatalogObjectTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CatalogName(String);

impl CatalogName {
    /// Creates a catalog name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogObjectTextError::EmptyCatalogName`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CatalogObjectTextError> {
        non_empty_text(value, CatalogObjectTextError::EmptyCatalogName).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CatalogName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CatalogName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CatalogName {
    type Err = CatalogObjectTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CatalogObjectId(String);

impl CatalogObjectId {
    /// Creates a catalog object identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogObjectTextError::EmptyCatalogObjectId`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CatalogObjectTextError> {
        non_empty_text(value, CatalogObjectTextError::EmptyCatalogObjectId).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CatalogObjectId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CatalogObjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CatalogObjectId {
    type Err = CatalogObjectTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CatalogDesignation(String);

impl CatalogDesignation {
    /// Creates a catalog designation from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CatalogObjectTextError::EmptyDesignation`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CatalogObjectTextError> {
        non_empty_text(value, CatalogObjectTextError::EmptyDesignation).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CatalogDesignation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CatalogDesignation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CatalogDesignation {
    type Err = CatalogObjectTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CatalogObjectKind {
    Messier,
    Ngc,
    Ic,
    Hipparcos,
    Gaia,
    HenryDraper,
    Simbad,
    ExoplanetCatalog,
    Unknown,
    Custom(String),
}

impl fmt::Display for CatalogObjectKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Messier => formatter.write_str("messier"),
            Self::Ngc => formatter.write_str("ngc"),
            Self::Ic => formatter.write_str("ic"),
            Self::Hipparcos => formatter.write_str("hipparcos"),
            Self::Gaia => formatter.write_str("gaia"),
            Self::HenryDraper => formatter.write_str("henry-draper"),
            Self::Simbad => formatter.write_str("simbad"),
            Self::ExoplanetCatalog => formatter.write_str("exoplanet-catalog"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogObjectKindParseError {
    Empty,
}

impl fmt::Display for CatalogObjectKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("catalog object kind cannot be empty"),
        }
    }
}

impl Error for CatalogObjectKindParseError {}

impl FromStr for CatalogObjectKind {
    type Err = CatalogObjectKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CatalogObjectKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "messier" => Ok(Self::Messier),
            "ngc" => Ok(Self::Ngc),
            "ic" => Ok(Self::Ic),
            "hipparcos" => Ok(Self::Hipparcos),
            "gaia" => Ok(Self::Gaia),
            "henry-draper" | "henrydraper" => Ok(Self::HenryDraper),
            "simbad" => Ok(Self::Simbad),
            "exoplanet-catalog" | "exoplanetcatalog" => Ok(Self::ExoplanetCatalog),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CatalogDesignation, CatalogName, CatalogObjectId, CatalogObjectKind, CatalogObjectTextError,
    };

    #[test]
    fn valid_catalog_name() {
        let name = CatalogName::new("Messier").unwrap();

        assert_eq!(name.as_str(), "Messier");
    }

    #[test]
    fn empty_catalog_name_rejected() {
        assert_eq!(
            CatalogName::new("  "),
            Err(CatalogObjectTextError::EmptyCatalogName)
        );
    }

    #[test]
    fn valid_catalog_object_id() {
        let object_id = CatalogObjectId::new("031").unwrap();

        assert_eq!(object_id.as_str(), "031");
    }

    #[test]
    fn empty_catalog_object_id_rejected() {
        assert_eq!(
            CatalogObjectId::new(" "),
            Err(CatalogObjectTextError::EmptyCatalogObjectId)
        );
    }

    #[test]
    fn catalog_object_kind_display_and_parse() {
        assert_eq!(CatalogObjectKind::Messier.to_string(), "messier");
        assert_eq!(
            "ngc".parse::<CatalogObjectKind>().unwrap(),
            CatalogObjectKind::Ngc
        );
    }

    #[test]
    fn custom_catalog_object_kind() {
        assert_eq!(
            "caldwell".parse::<CatalogObjectKind>().unwrap(),
            CatalogObjectKind::Custom("caldwell".to_string())
        );
    }

    #[test]
    fn designation_construction() {
        let designation = CatalogDesignation::new("Messier 31").unwrap();

        assert_eq!(designation.as_str(), "Messier 31");
    }
}
