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
pub enum MagnitudeError {
    NonFiniteMagnitude,
    NonFiniteColorIndex,
}

impl fmt::Display for MagnitudeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteMagnitude => formatter.write_str("magnitude must be finite"),
            Self::NonFiniteColorIndex => formatter.write_str("color index must be finite"),
        }
    }
}

impl Error for MagnitudeError {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Magnitude(f64);

impl Magnitude {
    /// Creates a magnitude from a finite numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`MagnitudeError::NonFiniteMagnitude`] when `value` is not finite.
    pub const fn new(value: f64) -> Result<Self, MagnitudeError> {
        if !value.is_finite() {
            return Err(MagnitudeError::NonFiniteMagnitude);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Magnitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MagnitudeKind {
    Apparent,
    Absolute,
    Bolometric,
    Visual,
    Photographic,
    Unknown,
    Custom(String),
}

impl fmt::Display for MagnitudeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Apparent => formatter.write_str("apparent"),
            Self::Absolute => formatter.write_str("absolute"),
            Self::Bolometric => formatter.write_str("bolometric"),
            Self::Visual => formatter.write_str("visual"),
            Self::Photographic => formatter.write_str("photographic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MagnitudeKindParseError {
    Empty,
}

impl fmt::Display for MagnitudeKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("magnitude kind cannot be empty"),
        }
    }
}

impl Error for MagnitudeKindParseError {}

impl FromStr for MagnitudeKind {
    type Err = MagnitudeKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(MagnitudeKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "apparent" => Ok(Self::Apparent),
            "absolute" => Ok(Self::Absolute),
            "bolometric" => Ok(Self::Bolometric),
            "visual" => Ok(Self::Visual),
            "photographic" => Ok(Self::Photographic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ColorIndex(f64);

impl ColorIndex {
    /// Creates a color index from a finite numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`MagnitudeError::NonFiniteColorIndex`] when `value` is not finite.
    pub const fn new(value: f64) -> Result<Self, MagnitudeError> {
        if !value.is_finite() {
            return Err(MagnitudeError::NonFiniteColorIndex);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ColorIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorIndex, Magnitude, MagnitudeError, MagnitudeKind};

    #[test]
    fn valid_positive_magnitude() {
        let magnitude = Magnitude::new(2.1).unwrap();

        assert!((magnitude.value() - 2.1).abs() < f64::EPSILON);
    }

    #[test]
    fn valid_negative_magnitude() {
        let magnitude = Magnitude::new(-1.46).unwrap();

        assert!((magnitude.value() - -1.46).abs() < f64::EPSILON);
    }

    #[test]
    fn magnitude_kind_display_and_parse() {
        assert_eq!(MagnitudeKind::Apparent.to_string(), "apparent");
        assert_eq!(
            "visual".parse::<MagnitudeKind>().unwrap(),
            MagnitudeKind::Visual
        );
    }

    #[test]
    fn custom_magnitude_kind() {
        assert_eq!(
            "infrared-band".parse::<MagnitudeKind>().unwrap(),
            MagnitudeKind::Custom("infrared-band".to_string())
        );
    }

    #[test]
    fn color_index_construction() {
        let color_index = ColorIndex::new(0.65).unwrap();

        assert!((color_index.value() - 0.65).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_non_finite_values() {
        assert_eq!(
            Magnitude::new(f64::NAN),
            Err(MagnitudeError::NonFiniteMagnitude)
        );
        assert_eq!(
            ColorIndex::new(f64::INFINITY),
            Err(MagnitudeError::NonFiniteColorIndex)
        );
    }
}
