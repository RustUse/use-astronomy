# use-moon

Primitive natural satellite vocabulary.

`use-moon` models non-empty moon names, descriptive moon kinds, and simple parent-child satellite relations. It does not simulate moon orbits, fetch satellite data, calculate tides, or infer formation history.

```rust
use use_moon::{MoonKind, MoonName, SatelliteRelation};

let name = MoonName::new("Europa").unwrap();
let relation = SatelliteRelation::new("Jupiter", "Europa").unwrap();

assert_eq!(name.as_str(), "Europa");
assert_eq!(relation.parent_identifier(), "Jupiter");
assert_eq!(MoonKind::Regular.to_string(), "regular");
```
