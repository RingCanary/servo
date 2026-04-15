## 2024-05-24 - [Avoid O(N^2) Vector lookups in DOM Iterators]
**Learning:** In components/script/dom (e.g. HTMLCollection, NamedNodeMap), iterating over DOM elements to construct unique `SupportedPropertyNames` lists using `vec.contains()` results in O(N^2) complexity. This is exacerbated by redundant string allocations (e.g. `DOMString::from`).
**Action:** Use `std::collections::HashSet` with O(1) `.insert()` checks for tracking uniqueness via `Atom` or `&str`, then instantiate `DOMString` only when an element is verified as unique.
