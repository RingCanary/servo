## 2025-02-12 - HTMLFormElement Property Deduction Speedup
**Learning:** In `components/script`, deduplicating `Atom`s (or `DOMString`s) using `names_vec.iter().any()` inside a loop introduces O(N^2) complexity. Furthermore, converting `Atom` to string to check if it is empty (`.to_string().is_empty()`) unnecessarily allocates memory on the heap.
**Action:** Always use `std::collections::HashSet` with pre-allocated capacity (`Vec::with_capacity`) for O(N) deduplication of collections. Call `.is_empty()` directly on `Atom` implementations to skip the `.to_string()` allocation step.
