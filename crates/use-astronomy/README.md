# use-astronomy

Facade crate for primitive RustUse astronomy vocabulary.

`use-astronomy` re-exports focused crates for celestial bodies, stars, planets, moons, orbits, orbital elements, astronomical coordinates, magnitudes, epochs, constellations, observations, and catalog objects.

`use-astronomy` is not an orbital simulator, telescope-control system, ephemeris engine, astrophysics framework, sky-map renderer, observatory system, or astronomy database client. It describes astronomy concepts; it does not simulate, fetch, render, or control them.

```rust
use use_astronomy::{
    astronomical_coordinate, celestial_body, constellation, epoch, magnitude, orbit,
    orbital_element, planet, star,
};

let body_kind = celestial_body::CelestialBodyKind::Planet;
let spectral_class = star::SpectralClass::G;
let planet_kind = planet::PlanetKind::Terrestrial;
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
let constellation_abbreviation = constellation::ConstellationAbbreviation::new("Lyr").unwrap();

assert_eq!(body_kind.to_string(), "planet");
assert_eq!(spectral_class.to_string(), "g");
assert_eq!(planet_kind.to_string(), "terrestrial");
assert_eq!(orbit_kind.to_string(), "heliocentric");
assert_eq!(eccentricity.kind(), &orbital_element::OrbitalElementKind::Eccentricity);
assert_eq!(coordinate.system(), &astronomical_coordinate::CoordinateSystem::ICRS);
assert_eq!(visual_magnitude.value(), -26.74);
assert_eq!(astronomical_epoch.label(), "J2000");
assert_eq!(constellation_abbreviation.as_str(), "Lyr");
```
