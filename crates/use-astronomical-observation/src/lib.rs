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
pub enum AstronomicalObservationTextError {
    EmptyObservationId,
}

impl fmt::Display for AstronomicalObservationTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservationId => {
                formatter.write_str("astronomical observation identifier cannot be empty")
            },
        }
    }
}

impl Error for AstronomicalObservationTextError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AstronomicalObservationId(String);

impl AstronomicalObservationId {
    /// Creates an astronomical observation identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`AstronomicalObservationTextError::EmptyObservationId`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AstronomicalObservationTextError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            Err(AstronomicalObservationTextError::EmptyObservationId)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AstronomicalObservationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AstronomicalObservationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AstronomicalObservationId {
    type Err = AstronomicalObservationTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationKind {
    Visual,
    Photometric,
    Spectroscopic,
    Astrometric,
    Radio,
    Infrared,
    Ultraviolet,
    XRay,
    GammaRay,
    Unknown,
    Custom(String),
}

impl fmt::Display for ObservationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visual => formatter.write_str("visual"),
            Self::Photometric => formatter.write_str("photometric"),
            Self::Spectroscopic => formatter.write_str("spectroscopic"),
            Self::Astrometric => formatter.write_str("astrometric"),
            Self::Radio => formatter.write_str("radio"),
            Self::Infrared => formatter.write_str("infrared"),
            Self::Ultraviolet => formatter.write_str("ultraviolet"),
            Self::XRay => formatter.write_str("x-ray"),
            Self::GammaRay => formatter.write_str("gamma-ray"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationKindParseError {
    Empty,
}

impl fmt::Display for ObservationKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("observation kind cannot be empty"),
        }
    }
}

impl Error for ObservationKindParseError {}

impl FromStr for ObservationKind {
    type Err = ObservationKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ObservationKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "visual" => Ok(Self::Visual),
            "photometric" => Ok(Self::Photometric),
            "spectroscopic" => Ok(Self::Spectroscopic),
            "astrometric" => Ok(Self::Astrometric),
            "radio" => Ok(Self::Radio),
            "infrared" => Ok(Self::Infrared),
            "ultraviolet" => Ok(Self::Ultraviolet),
            "x-ray" | "xray" => Ok(Self::XRay),
            "gamma-ray" | "gammaray" => Ok(Self::GammaRay),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationBand {
    Radio,
    Microwave,
    Infrared,
    Visible,
    Ultraviolet,
    XRay,
    GammaRay,
    Unknown,
    Custom(String),
}

impl fmt::Display for ObservationBand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Radio => formatter.write_str("radio"),
            Self::Microwave => formatter.write_str("microwave"),
            Self::Infrared => formatter.write_str("infrared"),
            Self::Visible => formatter.write_str("visible"),
            Self::Ultraviolet => formatter.write_str("ultraviolet"),
            Self::XRay => formatter.write_str("x-ray"),
            Self::GammaRay => formatter.write_str("gamma-ray"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationBandParseError {
    Empty,
}

impl fmt::Display for ObservationBandParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("observation band cannot be empty"),
        }
    }
}

impl Error for ObservationBandParseError {}

impl FromStr for ObservationBand {
    type Err = ObservationBandParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ObservationBandParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "radio" => Ok(Self::Radio),
            "microwave" => Ok(Self::Microwave),
            "infrared" => Ok(Self::Infrared),
            "visible" => Ok(Self::Visible),
            "ultraviolet" => Ok(Self::Ultraviolet),
            "x-ray" | "xray" => Ok(Self::XRay),
            "gamma-ray" | "gammaray" => Ok(Self::GammaRay),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationInstrumentKind {
    Telescope,
    RadioTelescope,
    Spectrograph,
    Camera,
    Photometer,
    Interferometer,
    SpaceTelescope,
    Unknown,
    Custom(String),
}

impl fmt::Display for ObservationInstrumentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Telescope => formatter.write_str("telescope"),
            Self::RadioTelescope => formatter.write_str("radio-telescope"),
            Self::Spectrograph => formatter.write_str("spectrograph"),
            Self::Camera => formatter.write_str("camera"),
            Self::Photometer => formatter.write_str("photometer"),
            Self::Interferometer => formatter.write_str("interferometer"),
            Self::SpaceTelescope => formatter.write_str("space-telescope"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationInstrumentKindParseError {
    Empty,
}

impl fmt::Display for ObservationInstrumentKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("observation instrument kind cannot be empty"),
        }
    }
}

impl Error for ObservationInstrumentKindParseError {}

impl FromStr for ObservationInstrumentKind {
    type Err = ObservationInstrumentKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ObservationInstrumentKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "telescope" => Ok(Self::Telescope),
            "radio-telescope" | "radiotelescope" => Ok(Self::RadioTelescope),
            "spectrograph" => Ok(Self::Spectrograph),
            "camera" => Ok(Self::Camera),
            "photometer" => Ok(Self::Photometer),
            "interferometer" => Ok(Self::Interferometer),
            "space-telescope" | "spacetelescope" => Ok(Self::SpaceTelescope),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SeeingCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Unknown,
    Custom(String),
}

impl fmt::Display for SeeingCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Excellent => formatter.write_str("excellent"),
            Self::Good => formatter.write_str("good"),
            Self::Fair => formatter.write_str("fair"),
            Self::Poor => formatter.write_str("poor"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeeingConditionParseError {
    Empty,
}

impl fmt::Display for SeeingConditionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("seeing condition cannot be empty"),
        }
    }
}

impl Error for SeeingConditionParseError {}

impl FromStr for SeeingCondition {
    type Err = SeeingConditionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SeeingConditionParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "excellent" => Ok(Self::Excellent),
            "good" => Ok(Self::Good),
            "fair" => Ok(Self::Fair),
            "poor" => Ok(Self::Poor),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AstronomicalObservationId, AstronomicalObservationTextError, ObservationBand,
        ObservationInstrumentKind, ObservationKind,
    };

    #[test]
    fn valid_observation_id() {
        let identifier = AstronomicalObservationId::new("obs-42").unwrap();

        assert_eq!(identifier.as_str(), "obs-42");
    }

    #[test]
    fn empty_observation_id_rejected() {
        assert_eq!(
            AstronomicalObservationId::new("   "),
            Err(AstronomicalObservationTextError::EmptyObservationId)
        );
    }

    #[test]
    fn observation_kind_display_and_parse() {
        assert_eq!(ObservationKind::Photometric.to_string(), "photometric");
        assert_eq!(
            "infrared".parse::<ObservationKind>().unwrap(),
            ObservationKind::Infrared
        );
    }

    #[test]
    fn observation_band_display_and_parse() {
        assert_eq!(ObservationBand::Visible.to_string(), "visible");
        assert_eq!(
            "x-ray".parse::<ObservationBand>().unwrap(),
            ObservationBand::XRay
        );
    }

    #[test]
    fn instrument_kind_display_and_parse() {
        assert_eq!(
            ObservationInstrumentKind::Telescope.to_string(),
            "telescope"
        );
        assert_eq!(
            "space telescope"
                .parse::<ObservationInstrumentKind>()
                .unwrap(),
            ObservationInstrumentKind::SpaceTelescope
        );
    }

    #[test]
    fn custom_observation_kind() {
        assert_eq!(
            "polarimetric".parse::<ObservationKind>().unwrap(),
            ObservationKind::Custom("polarimetric".to_string())
        );
    }
}
