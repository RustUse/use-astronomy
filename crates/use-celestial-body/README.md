# use-celestial-body

Primitive celestial body vocabulary.

`use-celestial-body` models non-empty celestial body names and identifiers, descriptive body kinds, and descriptive status labels. It does not fetch celestial object records, validate against external catalogs, infer classifications, or simulate motion.

```rust
use use_celestial_body::{
    CelestialBodyId, CelestialBodyKind, CelestialBodyName, CelestialBodyStatus,
};

let name = CelestialBodyName::new("Alpha Centauri A").unwrap();
let identifier = CelestialBodyId::new("HIP 71683").unwrap();

assert_eq!(name.as_str(), "Alpha Centauri A");
assert_eq!(identifier.as_str(), "HIP 71683");
assert_eq!(CelestialBodyKind::Star.to_string(), "star");
assert_eq!(CelestialBodyStatus::Confirmed.to_string(), "confirmed");
```
