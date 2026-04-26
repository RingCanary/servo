
## 2024-05-23 - Avoid iter().count() on ref-counted collections
**Learning:** Iterating over collections of DOM elements (like `IFrameCollection`) just to count them (`iter().count()`) introduces significant atomic reference-counting overhead (O(N) operations to create `DomRoot` elements), compared to directly delegating to the underlying vector length in O(1).
**Action:** When working with custom collection structs wrapping vectors, implement `len()` and `is_empty()` to expose the inner `Vec` length directly, bypassing iterator allocations.
