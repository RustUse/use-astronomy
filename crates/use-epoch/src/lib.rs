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
pub enum EpochError {
    EmptyLabel,
    NonFiniteJulianDate,
    NonFiniteModifiedJulianDate,
}

impl fmt::Display for EpochError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabel => formatter.write_str("astronomical epoch label cannot be empty"),
            Self::NonFiniteJulianDate => formatter.write_str("Julian date must be finite"),
            Self::NonFiniteModifiedJulianDate => {
                formatter.write_str("modified Julian date must be finite")
            },
        }
    }
}

impl Error for EpochError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EpochKind {
    J2000,
    B1950,
    Julian,
    Besselian,
    Observation,
    Unknown,
    Custom(String),
}

impl fmt::Display for EpochKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::J2000 => formatter.write_str("j2000"),
            Self::B1950 => formatter.write_str("b1950"),
            Self::Julian => formatter.write_str("julian"),
            Self::Besselian => formatter.write_str("besselian"),
            Self::Observation => formatter.write_str("observation"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpochKindParseError {
    Empty,
}

impl fmt::Display for EpochKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("epoch kind cannot be empty"),
        }
    }
}

impl Error for EpochKindParseError {}

impl FromStr for EpochKind {
    type Err = EpochKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(EpochKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "j2000" => Ok(Self::J2000),
            "b1950" => Ok(Self::B1950),
            "julian" => Ok(Self::Julian),
            "besselian" => Ok(Self::Besselian),
            "observation" => Ok(Self::Observation),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct JulianDate(f64);

impl JulianDate {
    /// Creates a Julian date from a finite numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`EpochError::NonFiniteJulianDate`] when `value` is not finite.
    pub const fn new(value: f64) -> Result<Self, EpochError> {
        if !value.is_finite() {
            return Err(EpochError::NonFiniteJulianDate);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ModifiedJulianDate(f64);

impl ModifiedJulianDate {
    /// Creates a modified Julian date from a finite numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`EpochError::NonFiniteModifiedJulianDate`] when `value` is not finite.
    pub const fn new(value: f64) -> Result<Self, EpochError> {
        if !value.is_finite() {
            return Err(EpochError::NonFiniteModifiedJulianDate);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AstronomicalEpoch {
    label: String,
    kind: EpochKind,
}

impl AstronomicalEpoch {
    /// Creates an astronomical epoch with a non-empty label and kind.
    ///
    /// # Errors
    ///
    /// Returns [`EpochError::EmptyLabel`] when the trimmed label is empty.
    pub fn new(label: impl AsRef<str>, kind: EpochKind) -> Result<Self, EpochError> {
        let trimmed = label.as_ref().trim();

        if trimmed.is_empty() {
            return Err(EpochError::EmptyLabel);
        }

        Ok(Self {
            label: trimmed.to_string(),
            kind,
        })
    }

    #[must_use]
    pub fn j2000() -> Self {
        Self {
            label: "J2000".to_string(),
            kind: EpochKind::J2000,
        }
    }

    #[must_use]
    pub fn b1950() -> Self {
        Self {
            label: "B1950".to_string(),
            kind: EpochKind::B1950,
        }
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn kind(&self) -> &EpochKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{AstronomicalEpoch, EpochKind, JulianDate, ModifiedJulianDate};

    #[test]
    fn epoch_kind_display_and_parse() {
        assert_eq!(EpochKind::J2000.to_string(), "j2000");
        assert_eq!(
            "observation".parse::<EpochKind>().unwrap(),
            EpochKind::Observation
        );
    }

    #[test]
    fn custom_epoch_kind() {
        assert_eq!(
            "catalog-epoch".parse::<EpochKind>().unwrap(),
            EpochKind::Custom("catalog-epoch".to_string())
        );
    }

    #[test]
    fn julian_date_construction() {
        let julian_date = JulianDate::new(2_451_545.0).unwrap();

        assert!((julian_date.value() - 2_451_545.0).abs() < f64::EPSILON);
    }

    #[test]
    fn modified_julian_date_construction() {
        let modified_julian_date = ModifiedJulianDate::new(51_544.5).unwrap();

        assert!((modified_julian_date.value() - 51_544.5).abs() < f64::EPSILON);
    }

    #[test]
    fn known_epoch_labels() {
        assert_eq!(AstronomicalEpoch::j2000().label(), "J2000");
        assert_eq!(AstronomicalEpoch::b1950().label(), "B1950");
    }
}
