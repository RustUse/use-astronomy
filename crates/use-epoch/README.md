# use-epoch

Primitive astronomical epoch vocabulary.

`use-epoch` models descriptive epoch kinds, labeled astronomical epochs, Julian dates, and modified Julian dates. It does not convert UTC, TDB, TT, or TAI; calculate sidereal time; fetch ephemeris data; or implement precise astronomy time systems.

```rust
use use_epoch::{AstronomicalEpoch, EpochKind, JulianDate, ModifiedJulianDate};

let epoch = AstronomicalEpoch::j2000();
let julian_date = JulianDate::new(2_451_545.0).unwrap();
let modified_julian_date = ModifiedJulianDate::new(51_544.5).unwrap();

assert_eq!(epoch.kind(), &EpochKind::J2000);
assert_eq!(julian_date.value(), 2_451_545.0);
assert_eq!(modified_julian_date.value(), 51_544.5);
```
