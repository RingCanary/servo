## 2025-03-09 - O(N^2) Hashset Deduplication
**Learning:** O(N^2) lookups inside HTMLFormElement supported property names using `iter().any()` were significantly slowing down performance when iterating DOM lists.
**Action:** Replaced `names_vec.iter().any()` with an explicit `HashSet` lookup. `HashSet::with_capacity` is extremely helpful here.
