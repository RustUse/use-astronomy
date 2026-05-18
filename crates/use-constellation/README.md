# use-constellation

Primitive constellation vocabulary.

`use-constellation` models non-empty constellation names, abbreviations, and region labels. It does not maintain official constellation boundary data, map sky regions, validate against IAU data, or render star charts.

```rust
use use_constellation::{ConstellationAbbreviation, ConstellationName, ConstellationRegion};

let name = ConstellationName::new("Lyra").unwrap();
let abbreviation = ConstellationAbbreviation::new("Lyr").unwrap();
let region = ConstellationRegion::new("northern sky").unwrap();

assert_eq!(name.as_str(), "Lyra");
assert_eq!(abbreviation.as_str(), "Lyr");
assert_eq!(region.as_str(), "northern sky");
```
