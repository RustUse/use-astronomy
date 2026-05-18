# use-catalog-object

Primitive astronomical catalog object vocabulary.

`use-catalog-object` models catalog names, object identifiers, designations, and descriptive catalog object kinds. It does not fetch catalog records, validate against remote catalogs, resolve object aliases, or implement database clients.

```rust
use use_catalog_object::{
    CatalogDesignation, CatalogName, CatalogObjectId, CatalogObjectKind,
};

let name = CatalogName::new("Messier").unwrap();
let object_id = CatalogObjectId::new("031").unwrap();
let designation = CatalogDesignation::new("Messier 31").unwrap();

assert_eq!(name.as_str(), "Messier");
assert_eq!(object_id.as_str(), "031");
assert_eq!(designation.as_str(), "Messier 31");
assert_eq!(CatalogObjectKind::Messier.to_string(), "messier");
```
