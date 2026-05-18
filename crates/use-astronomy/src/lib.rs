#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_astronomical_coordinate as astronomical_coordinate;
pub use use_astronomical_observation as astronomical_observation;
pub use use_astronomical_orbit as orbit;
pub use use_catalog_object as catalog_object;
pub use use_celestial_body as celestial_body;
pub use use_constellation as constellation;
pub use use_epoch as epoch;
pub use use_magnitude as magnitude;
pub use use_moon as moon;
pub use use_orbital_element as orbital_element;
pub use use_planet as planet;
pub use use_star as star;

#[cfg(test)]
mod tests {
    use super::{
        astronomical_coordinate, astronomical_observation, catalog_object, celestial_body,
        constellation, epoch, magnitude, moon, orbit, orbital_element, planet, star,
    };

    #[test]
    fn facade_composes_astronomy_primitives_without_simulation() {
        let body_kind = celestial_body::CelestialBodyKind::Star;
        let spectral_class = star::SpectralClass::G;
        let planet_kind = planet::PlanetKind::Terrestrial;
        let moon_kind = moon::MoonKind::Regular;
        let orbit_kind = orbit::OrbitKind::Heliocentric;
        let eccentricity = orbital_element::OrbitalElement::new(
            orbital_element::OrbitalElementKind::Eccentricity,
            orbital_element::OrbitalElementValue::new(0.0167).unwrap(),
        )
        .unwrap();
        let coordinate = astronomical_coordinate::AstronomicalCoordinate::new(
            astronomical_coordinate::RightAscension::from_degrees(279.234_734_79).unwrap(),
            astronomical_coordinate::Declination::new(38.783_688_96).unwrap(),
            astronomical_coordinate::CoordinateFrame::Equatorial,
            astronomical_coordinate::CoordinateSystem::ICRS,
        );
        let visual_magnitude = magnitude::Magnitude::new(-26.74).unwrap();
        let astronomical_epoch = epoch::AstronomicalEpoch::j2000();
        let constellation_abbreviation =
            constellation::ConstellationAbbreviation::new("Lyr").unwrap();
        let observation_band = astronomical_observation::ObservationBand::Visible;
        let designation = catalog_object::CatalogDesignation::new("Messier 31").unwrap();

        assert_eq!(body_kind.to_string(), "star");
        assert_eq!(spectral_class.to_string(), "g");
        assert_eq!(planet_kind.to_string(), "terrestrial");
        assert_eq!(moon_kind.to_string(), "regular");
        assert_eq!(orbit_kind.to_string(), "heliocentric");
        assert_eq!(
            eccentricity.kind(),
            &orbital_element::OrbitalElementKind::Eccentricity
        );
        assert_eq!(
            coordinate.frame(),
            &astronomical_coordinate::CoordinateFrame::Equatorial
        );
        assert_eq!(visual_magnitude.value(), -26.74);
        assert_eq!(astronomical_epoch.label(), "J2000");
        assert_eq!(constellation_abbreviation.as_str(), "Lyr");
        assert_eq!(observation_band.to_string(), "visible");
        assert_eq!(designation.as_str(), "Messier 31");
    }
}
