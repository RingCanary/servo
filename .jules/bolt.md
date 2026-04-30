## 2025-02-23 - O(1) iframe counting optimization
**Learning:** Iterating over DOM collections that yield ref-counted elements (e.g., `iframes.iter()` mapping to `iframe.element.as_rooted()`) introduces O(N) atomic reference-counting overhead for simple counts.
**Action:** Prefer direct `.len()` lookups on the underlying data structures to achieve O(1) performance.
