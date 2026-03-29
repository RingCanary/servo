## 2024-05-24 - [IFrameCollection length lookup]
**Learning:** The `IFrameCollection` struct currently lacks `len()` and `is_empty()` methods, causing users to write `iframes().iter().count()` which iterates over the underlying vector and unnecessarily converts each item to `DomRoot` before discarding it.
**Action:** Expose `len()` and `is_empty()` on `IFrameCollection` for O(1) performance instead of O(N).
