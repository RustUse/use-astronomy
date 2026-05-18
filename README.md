# use-astronomy

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-astronomy` is a primitive astronomy vocabulary set for describing celestial bodies, stars, planets, moons, orbits, orbital elements, astronomical coordinates, magnitudes, epochs, constellations, observations, and catalog objects.

`use-astronomy` is not an orbital simulator, telescope-control system, ephemeris engine, astrophysics framework, sky-map renderer, observatory system, or astronomy database client. It complements `use-physics`, `use-geometry`, `use-time`, `use-measure`, `use-units`, `use-data`, and `use-validate` by describing astronomy concepts rather than simulating, fetching, rendering, resolving, observing, or analyzing them.

## Workspace Crates

- `use-astronomy`: facade-only crate that re-exports the focused astronomy vocabulary crates.
- `use-celestial-body`: celestial body names, identifiers, kinds, and status labels.
- `use-star`: star names, star kinds, spectral classes, luminosity classes, and descriptive stellar mass values.
- `use-planet`: planet names, kinds, statuses, and planetary system names.
- `use-moon`: moon names, moon kinds, and simple satellite relations.
- `crates/use-orbit` published as `use-astronomical-orbit`: descriptive orbit names, kinds, directions, and states.
- `use-orbital-element`: orbital element kinds, validated element values, and simple element sets.
- `use-astronomical-coordinate`: right ascension, declination, coordinate frames, systems, and descriptive coordinates.
- `use-magnitude`: magnitude values, magnitude kinds, and color index values.
- `use-epoch`: epoch kinds, labeled epochs, Julian dates, and modified Julian dates.
- `use-constellation`: constellation names, abbreviations, and region labels.
- `use-astronomical-observation`: observation identifiers, kinds, bands, instrument kinds, and seeing conditions.
- `use-catalog-object`: catalog names, object identifiers, designations, and catalog object kinds.

## Scope

- small focused crates
- primitives over frameworks
- few or no dependencies
- stable APIs
- Rust 2024 edition
- strong documentation
- meaningful tests
- composable exports
- dual MIT OR Apache-2.0 license
- no unnecessary macros
- no async
- no global runtime assumptions
- no unsafe code
- no network calls
- no database fetching
- no simulation
- no rendering
- no telescope hardware control

## Example

```rust
use use_astronomy::{
    astronomical_coordinate, astronomical_observation, catalog_object, celestial_body,
    constellation, epoch, magnitude, orbit, orbital_element, planet, star,
};

let body_kind = celestial_body::CelestialBodyKind::Star;
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
let observation_band = astronomical_observation::ObservationBand::Visible;
let designation = catalog_object::CatalogDesignation::new("Messier 31").unwrap();

assert_eq!(body_kind.to_string(), "star");
assert_eq!(spectral_class.to_string(), "g");
assert_eq!(planet_kind.to_string(), "terrestrial");
assert_eq!(orbit_kind.to_string(), "heliocentric");
assert_eq!(eccentricity.kind(), &orbital_element::OrbitalElementKind::Eccentricity);
assert_eq!(coordinate.frame(), &astronomical_coordinate::CoordinateFrame::Equatorial);
assert_eq!(visual_magnitude.value(), -26.74);
assert_eq!(astronomical_epoch.label(), "J2000");
assert_eq!(constellation_abbreviation.as_str(), "Lyr");
assert_eq!(observation_band.to_string(), "visible");
assert_eq!(designation.as_str(), "Messier 31");
```

This example describes astronomy concepts. It does not simulate orbits, fetch catalog records, render sky maps, control telescopes, or compute ephemerides.

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
