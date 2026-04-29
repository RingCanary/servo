
## 2024-05-30 - O(N) deduplication using HashSet for Atoms
**Learning:** In the Servo codebase, deduplicating vectors of strings or Atoms using `.iter().any(|x| x == val)` creates O(N^2) bottlenecks when resolving `SupportedPropertyNames` in DOM structures like `HTMLCollection`, `HTMLFormElement`, and `NamedNodeMap`.
**Action:** Always use `std::collections::HashSet` to track seen elements to achieve O(N) deduplication. When working with string interning abstractions like `Atom`, prefer calling `.clone()` to populate the `HashSet` since it's extremely cheap, and defer the heavy heap allocation of `DOMString::from` until after confirming the value is unique. Additionally, avoid `.to_string().is_empty()` checks on Atoms in favor of `.is_empty()` to save unnecessary heap allocations.
