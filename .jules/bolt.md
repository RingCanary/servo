## 2026-06-07 - IFrameCollection Length Optimization
**Learning:** Iterating over DOM collections that yield ref-counted elements (e.g., `iframes.iter()` mapping to `iframe.element.as_rooted()`) introduces O(N) atomic reference-counting overhead for simple counts.
**Action:** Expose `len()` and `is_empty()` methods on internal structures like `IFrameCollection` to allow direct O(1) length lookups on the underlying `Vec` without ref-count manipulation, reducing time complexity from O(N) to O(1).
