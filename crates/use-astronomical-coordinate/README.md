# use-astronomical-coordinate

Primitive astronomical coordinate vocabulary.

`use-astronomical-coordinate` stores right ascension in degrees within `0.0..=360.0`, stores declination within `-90.0..=90.0`, and models coordinate frames and systems used by astronomy software. It does not transform coordinate systems, apply precession or nutation, compute apparent positions, or render sky maps.

```rust
use use_astronomical_coordinate::{
    AstronomicalCoordinate, CoordinateFrame, CoordinateSystem, Declination, RightAscension,
};

let coordinate = AstronomicalCoordinate::new(
    RightAscension::from_degrees(279.234_734_79).unwrap(),
    Declination::new(38.783_688_96).unwrap(),
    CoordinateFrame::Equatorial,
    CoordinateSystem::ICRS,
);

assert_eq!(coordinate.frame(), &CoordinateFrame::Equatorial);
assert_eq!(coordinate.system(), &CoordinateSystem::ICRS);
```
