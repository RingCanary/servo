## 2024-05-24 - Efficient Collection Size Access
**Learning:** Iterating over DOM collections that yield ref-counted elements (e.g., `iframes.iter()` mapping to `iframe.element.as_rooted()`) introduces O(N) atomic reference-counting overhead for simple counts.
**Action:** When determining the length of collections backed by a `Vec` like `IFrameCollection`, expose a direct `.len()` method to access the vector length in O(1) time rather than traversing `iter().count()`.
