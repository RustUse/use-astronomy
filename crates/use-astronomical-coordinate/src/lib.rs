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
pub enum AstronomicalCoordinateError {
    NonFiniteRightAscension,
    InvalidRightAscension,
    NonFiniteDeclination,
    InvalidDeclination,
}

impl fmt::Display for AstronomicalCoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteRightAscension => formatter.write_str("right ascension must be finite"),
            Self::InvalidRightAscension => {
                formatter.write_str("right ascension must be within 0.0..=360.0 degrees")
            },
            Self::NonFiniteDeclination => formatter.write_str("declination must be finite"),
            Self::InvalidDeclination => {
                formatter.write_str("declination must be within -90.0..=90.0 degrees")
            },
        }
    }
}

impl Error for AstronomicalCoordinateError {}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RightAscension(f64);

impl RightAscension {
    /// Creates right ascension from finite degrees within `0.0..=360.0`.
    ///
    /// # Errors
    ///
    /// Returns [`AstronomicalCoordinateError::NonFiniteRightAscension`] when `value` is not finite,
    /// or [`AstronomicalCoordinateError::InvalidRightAscension`] when it is outside `0.0..=360.0`.
    pub fn from_degrees(value: f64) -> Result<Self, AstronomicalCoordinateError> {
        if !value.is_finite() {
            return Err(AstronomicalCoordinateError::NonFiniteRightAscension);
        }

        if !(0.0..=360.0).contains(&value) {
            return Err(AstronomicalCoordinateError::InvalidRightAscension);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.0
    }
}

impl fmt::Display for RightAscension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.degrees().fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Declination(f64);

impl Declination {
    /// Creates declination from finite degrees within `-90.0..=90.0`.
    ///
    /// # Errors
    ///
    /// Returns [`AstronomicalCoordinateError::NonFiniteDeclination`] when `value` is not finite,
    /// or [`AstronomicalCoordinateError::InvalidDeclination`] when it is outside `-90.0..=90.0`.
    pub fn new(value: f64) -> Result<Self, AstronomicalCoordinateError> {
        if !value.is_finite() {
            return Err(AstronomicalCoordinateError::NonFiniteDeclination);
        }

        if !(-90.0..=90.0).contains(&value) {
            return Err(AstronomicalCoordinateError::InvalidDeclination);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Declination {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.degrees().fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CoordinateFrame {
    Equatorial,
    Ecliptic,
    Galactic,
    Horizontal,
    Supergalactic,
    Unknown,
    Custom(String),
}

impl fmt::Display for CoordinateFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equatorial => formatter.write_str("equatorial"),
            Self::Ecliptic => formatter.write_str("ecliptic"),
            Self::Galactic => formatter.write_str("galactic"),
            Self::Horizontal => formatter.write_str("horizontal"),
            Self::Supergalactic => formatter.write_str("supergalactic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateFrameParseError {
    Empty,
}

impl fmt::Display for CoordinateFrameParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("coordinate frame cannot be empty"),
        }
    }
}

impl Error for CoordinateFrameParseError {}

impl FromStr for CoordinateFrame {
    type Err = CoordinateFrameParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CoordinateFrameParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "equatorial" => Ok(Self::Equatorial),
            "ecliptic" => Ok(Self::Ecliptic),
            "galactic" => Ok(Self::Galactic),
            "horizontal" => Ok(Self::Horizontal),
            "supergalactic" => Ok(Self::Supergalactic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CoordinateSystem {
    ICRS,
    FK5,
    J2000,
    B1950,
    Apparent,
    Unknown,
    Custom(String),
}

impl fmt::Display for CoordinateSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ICRS => formatter.write_str("icrs"),
            Self::FK5 => formatter.write_str("fk5"),
            Self::J2000 => formatter.write_str("j2000"),
            Self::B1950 => formatter.write_str("b1950"),
            Self::Apparent => formatter.write_str("apparent"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateSystemParseError {
    Empty,
}

impl fmt::Display for CoordinateSystemParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("coordinate system cannot be empty"),
        }
    }
}

impl Error for CoordinateSystemParseError {}

impl FromStr for CoordinateSystem {
    type Err = CoordinateSystemParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CoordinateSystemParseError::Empty);
        }

        match trimmed.to_ascii_uppercase().as_str() {
            "ICRS" => Ok(Self::ICRS),
            "FK5" => Ok(Self::FK5),
            "J2000" => Ok(Self::J2000),
            "B1950" => Ok(Self::B1950),
            _ if normalized_key(trimmed) == "apparent" => Ok(Self::Apparent),
            _ if normalized_key(trimmed) == "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AstronomicalCoordinate {
    right_ascension: RightAscension,
    declination: Declination,
    frame: CoordinateFrame,
    system: CoordinateSystem,
}

impl AstronomicalCoordinate {
    #[must_use]
    pub const fn new(
        right_ascension: RightAscension,
        declination: Declination,
        frame: CoordinateFrame,
        system: CoordinateSystem,
    ) -> Self {
        Self {
            right_ascension,
            declination,
            frame,
            system,
        }
    }

    #[must_use]
    pub const fn right_ascension(&self) -> RightAscension {
        self.right_ascension
    }

    #[must_use]
    pub const fn declination(&self) -> Declination {
        self.declination
    }

    #[must_use]
    pub const fn frame(&self) -> &CoordinateFrame {
        &self.frame
    }

    #[must_use]
    pub const fn system(&self) -> &CoordinateSystem {
        &self.system
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AstronomicalCoordinate, AstronomicalCoordinateError, CoordinateFrame, CoordinateSystem,
        Declination, RightAscension,
    };

    #[test]
    fn valid_right_ascension() {
        let right_ascension = RightAscension::from_degrees(180.0).unwrap();

        assert!((right_ascension.degrees() - 180.0).abs() < f64::EPSILON);
    }

    #[test]
    fn invalid_right_ascension_rejected() {
        assert_eq!(
            RightAscension::from_degrees(361.0),
            Err(AstronomicalCoordinateError::InvalidRightAscension)
        );
    }

    #[test]
    fn valid_declination() {
        let declination = Declination::new(-16.7161).unwrap();

        assert!((declination.degrees() - -16.7161).abs() < f64::EPSILON);
    }

    #[test]
    fn invalid_declination_rejected() {
        assert_eq!(
            Declination::new(-91.0),
            Err(AstronomicalCoordinateError::InvalidDeclination)
        );
    }

    #[test]
    fn coordinate_frame_display_and_parse() {
        assert_eq!(CoordinateFrame::Galactic.to_string(), "galactic");
        assert_eq!(
            "horizontal".parse::<CoordinateFrame>().unwrap(),
            CoordinateFrame::Horizontal
        );
    }

    #[test]
    fn coordinate_system_display_and_parse() {
        assert_eq!(CoordinateSystem::ICRS.to_string(), "icrs");
        assert_eq!(
            "j2000".parse::<CoordinateSystem>().unwrap(),
            CoordinateSystem::J2000
        );
    }

    #[test]
    fn custom_coordinate_frame() {
        assert_eq!(
            "topocentric".parse::<CoordinateFrame>().unwrap(),
            CoordinateFrame::Custom("topocentric".to_string())
        );
    }

    #[test]
    fn astronomical_coordinate_construction() {
        let coordinate = AstronomicalCoordinate::new(
            RightAscension::from_degrees(279.234_734_79).unwrap(),
            Declination::new(38.783_688_96).unwrap(),
            CoordinateFrame::Equatorial,
            CoordinateSystem::ICRS,
        );

        assert_eq!(coordinate.frame(), &CoordinateFrame::Equatorial);
        assert_eq!(coordinate.system(), &CoordinateSystem::ICRS);
    }
}
