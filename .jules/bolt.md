## 2025-03-01 - Avoid allocating string on empty check & HashSet insertion over Iterator::any

**Learning:** `Atom`s in Servo can be checked for emptiness directly using `.is_empty()` instead of `.to_string().is_empty()` which triggers a heap allocation. Furthermore, deduplicating an array of `Atom`s or `DOMString`s is O(N^2) using `!vec.iter().any(|v| *v == x)`, but using a `HashSet` allows O(1) membership checks (O(N) overall), and storing references to the Atom `seen.insert(&elem.name)` instead of `.clone()`ing them into the HashSet is even faster (zero-cost abstraction).

**Action:** When filtering or deduplicating data, use `HashSet` with references `&T` to prevent unnecessary allocations, and ensure simple property checks (like emptiness) use the native implementations of the types rather than coercing them into strings first.
