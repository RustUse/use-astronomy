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

fn non_empty_text(value: impl AsRef<str>, error: StarError) -> Result<String, StarError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StarError {
    EmptyName,
    NonFiniteMass,
    NegativeMass,
}

impl fmt::Display for StarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("star name cannot be empty"),
            Self::NonFiniteMass => formatter.write_str("stellar mass must be finite"),
            Self::NegativeMass => formatter.write_str("stellar mass cannot be negative"),
        }
    }
}

impl Error for StarError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StarName(String);

impl StarName {
    /// Creates a star name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`StarError::EmptyName`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, StarError> {
        non_empty_text(value, StarError::EmptyName).map(Self)
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

impl AsRef<str> for StarName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for StarName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for StarName {
    type Err = StarError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StarKind {
    MainSequence,
    RedGiant,
    WhiteDwarf,
    NeutronStar,
    Protostar,
    Supergiant,
    BrownDwarf,
    Variable,
    Binary,
    Unknown,
    Custom(String),
}

impl fmt::Display for StarKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MainSequence => formatter.write_str("main-sequence"),
            Self::RedGiant => formatter.write_str("red-giant"),
            Self::WhiteDwarf => formatter.write_str("white-dwarf"),
            Self::NeutronStar => formatter.write_str("neutron-star"),
            Self::Protostar => formatter.write_str("protostar"),
            Self::Supergiant => formatter.write_str("supergiant"),
            Self::BrownDwarf => formatter.write_str("brown-dwarf"),
            Self::Variable => formatter.write_str("variable"),
            Self::Binary => formatter.write_str("binary"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StarKindParseError {
    Empty,
}

impl fmt::Display for StarKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("star kind cannot be empty"),
        }
    }
}

impl Error for StarKindParseError {}

impl FromStr for StarKind {
    type Err = StarKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(StarKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "main-sequence" | "mainsequence" => Ok(Self::MainSequence),
            "red-giant" | "redgiant" => Ok(Self::RedGiant),
            "white-dwarf" | "whitedwarf" => Ok(Self::WhiteDwarf),
            "neutron-star" | "neutronstar" => Ok(Self::NeutronStar),
            "protostar" => Ok(Self::Protostar),
            "supergiant" => Ok(Self::Supergiant),
            "brown-dwarf" | "browndwarf" => Ok(Self::BrownDwarf),
            "variable" => Ok(Self::Variable),
            "binary" => Ok(Self::Binary),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SpectralClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    L,
    T,
    Y,
    Unknown,
    Custom(String),
}

impl fmt::Display for SpectralClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::O => formatter.write_str("o"),
            Self::B => formatter.write_str("b"),
            Self::A => formatter.write_str("a"),
            Self::F => formatter.write_str("f"),
            Self::G => formatter.write_str("g"),
            Self::K => formatter.write_str("k"),
            Self::M => formatter.write_str("m"),
            Self::L => formatter.write_str("l"),
            Self::T => formatter.write_str("t"),
            Self::Y => formatter.write_str("y"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectralClassParseError {
    Empty,
}

impl fmt::Display for SpectralClassParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("spectral class cannot be empty"),
        }
    }
}

impl Error for SpectralClassParseError {}

impl FromStr for SpectralClass {
    type Err = SpectralClassParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(SpectralClassParseError::Empty);
        }

        match trimmed.to_ascii_uppercase().as_str() {
            "O" => Ok(Self::O),
            "B" => Ok(Self::B),
            "A" => Ok(Self::A),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "K" => Ok(Self::K),
            "M" => Ok(Self::M),
            "L" => Ok(Self::L),
            "T" => Ok(Self::T),
            "Y" => Ok(Self::Y),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LuminosityClass {
    Ia,
    Ib,
    II,
    III,
    IV,
    V,
    VI,
    Unknown,
    Custom(String),
}

impl fmt::Display for LuminosityClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ia => formatter.write_str("ia"),
            Self::Ib => formatter.write_str("ib"),
            Self::II => formatter.write_str("ii"),
            Self::III => formatter.write_str("iii"),
            Self::IV => formatter.write_str("iv"),
            Self::V => formatter.write_str("v"),
            Self::VI => formatter.write_str("vi"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LuminosityClassParseError {
    Empty,
}

impl fmt::Display for LuminosityClassParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("luminosity class cannot be empty"),
        }
    }
}

impl Error for LuminosityClassParseError {}

impl FromStr for LuminosityClass {
    type Err = LuminosityClassParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(LuminosityClassParseError::Empty);
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "ia" => Ok(Self::Ia),
            "ib" => Ok(Self::Ib),
            "ii" => Ok(Self::II),
            "iii" => Ok(Self::III),
            "iv" => Ok(Self::IV),
            "v" => Ok(Self::V),
            "vi" => Ok(Self::VI),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct StellarMass(f64);

impl StellarMass {
    /// Creates a stellar mass from a finite, non-negative solar-mass value.
    ///
    /// # Errors
    ///
    /// Returns [`StarError::NonFiniteMass`] when `value` is not finite, or
    /// [`StarError::NegativeMass`] when `value` is negative.
    pub const fn new(value: f64) -> Result<Self, StarError> {
        if !value.is_finite() {
            return Err(StarError::NonFiniteMass);
        }

        if value < 0.0 {
            return Err(StarError::NegativeMass);
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn solar_masses(self) -> f64 {
        self.0
    }
}

impl fmt::Display for StellarMass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.solar_masses().fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{LuminosityClass, SpectralClass, StarError, StarKind, StarName, StellarMass};

    #[test]
    fn valid_star_name() {
        let name = StarName::new("Sirius A").unwrap();

        assert_eq!(name.as_str(), "Sirius A");
    }

    #[test]
    fn empty_star_name_rejected() {
        assert_eq!(StarName::new("   "), Err(StarError::EmptyName));
    }

    #[test]
    fn star_kind_display_and_parse() {
        assert_eq!(StarKind::MainSequence.to_string(), "main-sequence");
        assert_eq!("red giant".parse::<StarKind>().unwrap(), StarKind::RedGiant);
    }

    #[test]
    fn spectral_class_display_and_parse() {
        assert_eq!(SpectralClass::G.to_string(), "g");
        assert_eq!("k".parse::<SpectralClass>().unwrap(), SpectralClass::K);
    }

    #[test]
    fn luminosity_class_display_and_parse() {
        assert_eq!(LuminosityClass::V.to_string(), "v");
        assert_eq!(
            "iii".parse::<LuminosityClass>().unwrap(),
            LuminosityClass::III
        );
    }

    #[test]
    fn valid_stellar_mass() {
        let mass = StellarMass::new(1.0).unwrap();

        assert!((mass.solar_masses() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn negative_stellar_mass_rejected() {
        assert_eq!(StellarMass::new(-0.1), Err(StarError::NegativeMass));
    }
}
