# use-star

Primitive star vocabulary.

`use-star` models star names, descriptive star kinds, spectral classes, luminosity classes, and simple stellar mass values expressed in solar masses. It does not model stellar evolution, calculate luminosity, fetch catalog data, or implement astrophysics formulas beyond simple validation.

```rust
use use_star::{LuminosityClass, SpectralClass, StarKind, StarName, StellarMass};

let name = StarName::new("Sirius A").unwrap();
let mass = StellarMass::new(2.063).unwrap();

assert_eq!(name.as_str(), "Sirius A");
assert_eq!(mass.solar_masses(), 2.063);
assert_eq!(StarKind::MainSequence.to_string(), "main-sequence");
assert_eq!(SpectralClass::A.to_string(), "a");
assert_eq!(LuminosityClass::V.to_string(), "v");
```
