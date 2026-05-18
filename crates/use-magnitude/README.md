# use-magnitude

Primitive astronomical magnitude vocabulary.

`use-magnitude` models validated magnitude values, descriptive magnitude kinds, and color index values. It does not calculate luminosity, calculate distances, perform photometric calibration, or fetch observation data.

```rust
use use_magnitude::{ColorIndex, Magnitude, MagnitudeKind};

let magnitude = Magnitude::new(-1.46).unwrap();
let color_index = ColorIndex::new(0.00).unwrap();

assert_eq!(magnitude.value(), -1.46);
assert_eq!(color_index.value(), 0.0);
assert_eq!(MagnitudeKind::Apparent.to_string(), "apparent");
```
