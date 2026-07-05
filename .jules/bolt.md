## 2025-07-05 - O(1) Length for IFrameCollection
**Learning:** `IFrameCollection` provides length through an iterator `.count()`, iterating over the underlying `Vec`, yielding O(N) operations. Because the elements are dynamically checked for root caching in `length()`, traversing takes O(N). But `IFrameCollection` already caches `IFrame` structures in `self.iframes: Vec<IFrame>`. The length is exactly `self.iframes.len()` in O(1).
**Action:** When a struct wraps a `Vec` and needs a length method, implement it directly using `len()` instead of exposing an iterator that users map/count over.
