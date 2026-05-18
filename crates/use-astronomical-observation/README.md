# use-astronomical-observation

Primitive astronomical observation vocabulary.

`use-astronomical-observation` models non-empty observation identifiers, descriptive observation kinds, bands, instrument kinds, and seeing conditions. It does not control telescopes, fetch observations, reduce data, process spectra, or calibrate images.

```rust
use use_astronomical_observation::{
    AstronomicalObservationId, ObservationBand, ObservationInstrumentKind, ObservationKind,
    SeeingCondition,
};

let identifier = AstronomicalObservationId::new("obs-42").unwrap();

assert_eq!(identifier.as_str(), "obs-42");
assert_eq!(ObservationKind::Photometric.to_string(), "photometric");
assert_eq!(ObservationBand::Visible.to_string(), "visible");
assert_eq!(ObservationInstrumentKind::Telescope.to_string(), "telescope");
assert_eq!(SeeingCondition::Good.to_string(), "good");
```
