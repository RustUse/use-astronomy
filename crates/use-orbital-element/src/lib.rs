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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrbitalElementKind {
    SemiMajorAxis,
    Eccentricity,
    Inclination,
    LongitudeOfAscendingNode,
    ArgumentOfPeriapsis,
    TrueAnomaly,
    MeanAnomaly,
    Epoch,
    Unknown,
    Custom(String),
}

impl fmt::Display for OrbitalElementKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SemiMajorAxis => formatter.write_str("semi-major-axis"),
            Self::Eccentricity => formatter.write_str("eccentricity"),
            Self::Inclination => formatter.write_str("inclination"),
            Self::LongitudeOfAscendingNode => formatter.write_str("longitude-of-ascending-node"),
            Self::ArgumentOfPeriapsis => formatter.write_str("argument-of-periapsis"),
            Self::TrueAnomaly => formatter.write_str("true-anomaly"),
            Self::MeanAnomaly => formatter.write_str("mean-anomaly"),
            Self::Epoch => formatter.write_str("epoch"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrbitalElementKindParseError {
    Empty,
}

impl fmt::Display for OrbitalElementKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("orbital element kind cannot be empty"),
        }
    }
}

impl Error for OrbitalElementKindParseError {}

impl FromStr for OrbitalElementKind {
    type Err = OrbitalElementKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrbitalElementKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "semi-major-axis" | "semimajoraxis" => Ok(Self::SemiMajorAxis),
            "eccentricity" => Ok(Self::Eccentricity),
            "inclination" => Ok(Self::Inclination),
            "longitude-of-ascending-node" | "loan" => Ok(Self::LongitudeOfAscendingNode),
            "argument-of-periapsis" | "aop" => Ok(Self::ArgumentOfPeriapsis),
            "true-anomaly" | "trueanomaly" => Ok(Self::TrueAnomaly),
            "mean-anomaly" | "meananomaly" => Ok(Self::MeanAnomaly),
            "epoch" => Ok(Self::Epoch),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrbitalElementValueError {
    NonFiniteValue,
    EmptyUnitLabel,
    NegativeEccentricity,
    InvalidInclination,
}

impl fmt::Display for OrbitalElementValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue => formatter.write_str("orbital element value must be finite"),
            Self::EmptyUnitLabel => {
                formatter.write_str("orbital element unit label cannot be empty")
            },
            Self::NegativeEccentricity => formatter.write_str("eccentricity cannot be negative"),
            Self::InvalidInclination => {
                formatter.write_str("inclination must be within 0.0..=180.0 degrees")
            },
        }
    }
}

impl Error for OrbitalElementValueError {}

#[derive(Clone, Debug, PartialEq)]
pub struct OrbitalElementValue {
    value: f64,
    unit_label: Option<String>,
}

impl OrbitalElementValue {
    pub fn new(value: f64) -> Result<Self, OrbitalElementValueError> {
        if !value.is_finite() {
            return Err(OrbitalElementValueError::NonFiniteValue);
        }

        Ok(Self {
            value,
            unit_label: None,
        })
    }

    pub fn with_unit(
        value: f64,
        unit_label: impl AsRef<str>,
    ) -> Result<Self, OrbitalElementValueError> {
        let unit_label = unit_label.as_ref().trim();
        if unit_label.is_empty() {
            return Err(OrbitalElementValueError::EmptyUnitLabel);
        }

        let mut element_value = Self::new(value)?;
        element_value.unit_label = Some(unit_label.to_string());
        Ok(element_value)
    }

    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn unit_label(&self) -> Option<&str> {
        self.unit_label.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrbitalElement {
    kind: OrbitalElementKind,
    value: OrbitalElementValue,
}

impl OrbitalElement {
    pub fn new(
        kind: OrbitalElementKind,
        value: OrbitalElementValue,
    ) -> Result<Self, OrbitalElementValueError> {
        match kind {
            OrbitalElementKind::Eccentricity if value.value() < 0.0 => {
                return Err(OrbitalElementValueError::NegativeEccentricity);
            },
            OrbitalElementKind::Inclination if !(0.0..=180.0).contains(&value.value()) => {
                return Err(OrbitalElementValueError::InvalidInclination);
            },
            _ => {},
        }

        Ok(Self { kind, value })
    }

    #[must_use]
    pub const fn kind(&self) -> &OrbitalElementKind {
        &self.kind
    }

    #[must_use]
    pub const fn value(&self) -> &OrbitalElementValue {
        &self.value
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OrbitalElementSet {
    elements: Vec<OrbitalElement>,
}

impl OrbitalElementSet {
    #[must_use]
    pub fn new(elements: Vec<OrbitalElement>) -> Self {
        Self { elements }
    }

    #[must_use]
    pub fn elements(&self) -> &[OrbitalElement] {
        &self.elements
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OrbitalElement, OrbitalElementKind, OrbitalElementSet, OrbitalElementValue,
        OrbitalElementValueError,
    };

    #[test]
    fn orbital_element_kind_display_and_parse() {
        assert_eq!(
            OrbitalElementKind::SemiMajorAxis.to_string(),
            "semi-major-axis"
        );
        assert_eq!(
            "mean anomaly".parse::<OrbitalElementKind>().unwrap(),
            OrbitalElementKind::MeanAnomaly
        );
    }

    #[test]
    fn custom_orbital_element_kind() {
        assert_eq!(
            "perihelion-time".parse::<OrbitalElementKind>().unwrap(),
            OrbitalElementKind::Custom("perihelion-time".to_string())
        );
    }

    #[test]
    fn valid_eccentricity() {
        let element = OrbitalElement::new(
            OrbitalElementKind::Eccentricity,
            OrbitalElementValue::new(0.0167).unwrap(),
        )
        .unwrap();

        assert_eq!(element.value().value(), 0.0167);
    }

    #[test]
    fn negative_eccentricity_rejected() {
        assert_eq!(
            OrbitalElement::new(
                OrbitalElementKind::Eccentricity,
                OrbitalElementValue::new(-0.1).unwrap(),
            ),
            Err(OrbitalElementValueError::NegativeEccentricity)
        );
    }

    #[test]
    fn valid_inclination() {
        let element = OrbitalElement::new(
            OrbitalElementKind::Inclination,
            OrbitalElementValue::with_unit(98.7, "deg").unwrap(),
        )
        .unwrap();

        assert_eq!(element.value().unit_label(), Some("deg"));
    }

    #[test]
    fn invalid_inclination_rejected() {
        assert_eq!(
            OrbitalElement::new(
                OrbitalElementKind::Inclination,
                OrbitalElementValue::new(181.0).unwrap(),
            ),
            Err(OrbitalElementValueError::InvalidInclination)
        );
    }

    #[test]
    fn orbital_element_set_construction() {
        let elements = vec![
            OrbitalElement::new(
                OrbitalElementKind::SemiMajorAxis,
                OrbitalElementValue::with_unit(1.0, "AU").unwrap(),
            )
            .unwrap(),
            OrbitalElement::new(
                OrbitalElementKind::Eccentricity,
                OrbitalElementValue::new(0.0167).unwrap(),
            )
            .unwrap(),
        ];

        let set = OrbitalElementSet::new(elements);

        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
    }
}
