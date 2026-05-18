## 2024-05-24 - HTMLFormElement Deduplication Bottleneck
**Learning:** Checking for string emptiness using `!atom.to_string().is_empty()` allocates a new heap String, which is expensive in loops. Furthermore, deduplicating elements using `.any()` inside a loop creates an O(N^2) bottleneck.
**Action:** Always use the native `.is_empty()` on `Atom`s. Use `HashSet` for deduplication, and pre-allocate vectors when the source length is known.
