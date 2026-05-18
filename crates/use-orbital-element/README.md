# use-orbital-element

Primitive orbital element vocabulary.

`use-orbital-element` models orbital element kinds, validated element values with optional unit labels, and simple orbital element sets. It does not propagate orbits, solve Kepler's equation, convert between anomaly types, or implement astrodynamics.

```rust
use use_orbital_element::{OrbitalElement, OrbitalElementKind, OrbitalElementValue};

let eccentricity = OrbitalElement::new(
    OrbitalElementKind::Eccentricity,
    OrbitalElementValue::new(0.0167).unwrap(),
)
.unwrap();

assert_eq!(eccentricity.kind(), &OrbitalElementKind::Eccentricity);
assert_eq!(eccentricity.value().value(), 0.0167);
```
