## 2026-05-23 - [Optimize SupportedPropertyNames collection with HashSet]
**Learning:** In `components/script/dom/`, collecting unique strings (like IDs or names) for DOM collections previously relied on O(N^2) loops with `Vec::contains(&DOMString)`, causing unnecessary heap allocations and redundant array scans.
**Action:** Use `std::collections::HashSet` to track seen items using their native interned representations (e.g., `stylo_atoms::Atom` or `&str`), inserting into the results `Vec` only upon successful `HashSet::insert`. Defer `DOMString::from` conversion until uniqueness is verified.
