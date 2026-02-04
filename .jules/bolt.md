## 2025-10-30 - O(N^2) Lookup in DOM Collections
**Learning:** `Vec::contains` in iteration loops over DOM collections (like `HTMLCollection::SupportedPropertyNames`) causes O(N^2) complexity. This is particularly bad for pages with many named elements.
**Action:** Use `HashSet` (or `HashSet<Atom>`) for O(1) lookups to track seen elements when iterating, ensuring O(N) complexity.
