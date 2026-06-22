
## 2024-05-18 - HTMLFormElement Collection Optimization
**Learning:** Using `HashSet::with_capacity` to deduplicate nodes in `HTMLFormElement::SupportedPropertyNames` reduces O(N^2) loops to O(N). Avoiding `.to_string().is_empty()` on `Atom` prevents unnecessary string allocations.
**Action:** When filtering or deduplicating DOM node collections based on `Atom` keys, use `!atom.is_empty()` instead of converting to strings, and use `HashSet` with `insert` to populate unique vectors in O(N) time.
