# use-planet

Primitive planet vocabulary.

`use-planet` models non-empty planet names, descriptive planet kinds, descriptive planet statuses, and non-empty planetary system names. It does not validate official planet status, fetch exoplanet catalog data, simulate planetary systems, or calculate habitability.

```rust
use use_planet::{PlanetKind, PlanetName, PlanetStatus, PlanetarySystemName};

let name = PlanetName::new("Kepler-452b").unwrap();
let system = PlanetarySystemName::new("Kepler-452").unwrap();

assert_eq!(name.as_str(), "Kepler-452b");
assert_eq!(system.as_str(), "Kepler-452");
assert_eq!(PlanetKind::Exoplanet.to_string(), "exoplanet");
assert_eq!(PlanetStatus::Confirmed.to_string(), "confirmed");
```
