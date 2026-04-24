## 2026-04-24 - O(N^2) Vector lookups in DOM getters
**Learning:** In the DOM script component, checking uniqueness for returned arrays (e.g. `SupportedPropertyNames`) using `!vec.contains(&DOMString)` results in (N^2)$ string comparisons and necessitates generating a heap-allocated `DOMString` for every item, even duplicates.
**Action:** Replace linear vector scans for uniqueness with a `HashSet` holding cheap-to-clone interned string `Atom`s or `&str`s, reducing time complexity to (N)$ and preventing heap allocations of `DOMString` until an element is confirmed to be unique.
