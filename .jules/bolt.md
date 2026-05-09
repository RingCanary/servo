## 2024-03-24 - HTMLCollection / HTMLFormElement DOMString Dedup
**Learning:** In the script component, `SupportedPropertyNames` implementation manually checked for duplicates using `Vec::contains` on potentially long O(N) lists, which scales as O(N^2) while incurring heap allocations wrapping `Atom`s into `DOMString` prematurely.
**Action:** Always pre-filter uniqueness over `Atom` (which implements `Eq` + `Hash`) using a `HashSet` to skip duplicate string conversions and turn the deduplication into an O(N) process.
