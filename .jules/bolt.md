## 2024-05-19 - [O(N^2) Vector Iteration during Deduplication]
**Learning:** Found multiple instances where DOM collections (like `HTMLCollection`, `HTMLFormElement`, `NamedNodeMap`) deduplicate strings by iterating over an expanding vector using `.contains()` or `.iter().any()`, causing O(N^2) lookup complexity.
**Action:** When filtering unique names, use a `std::collections::HashSet` to track seen `Atom`s (or `&str` slices). `Atom`s are cheap to clone, reducing lookup time to O(1) and allowing us to defer allocating `DOMString`s until uniqueness is confirmed.
