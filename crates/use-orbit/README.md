# use-astronomical-orbit

Primitive orbit vocabulary.

`use-astronomical-orbit` models non-empty orbit names, descriptive orbit kinds, orbit directions, and orbit states. It does not implement orbital mechanics, propagate orbits, calculate trajectories, or simulate n-body systems.

```rust
use use_astronomical_orbit::{OrbitDirection, OrbitKind, OrbitName, OrbitState};

let name = OrbitName::new("Earth heliocentric orbit").unwrap();

assert_eq!(name.as_str(), "Earth heliocentric orbit");
assert_eq!(OrbitKind::Heliocentric.to_string(), "heliocentric");
assert_eq!(OrbitDirection::Prograde.to_string(), "prograde");
assert_eq!(OrbitState::Bound.to_string(), "bound");
```
