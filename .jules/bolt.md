
## $(date +%Y-%m-%d) - Optimize O(N^2) Form Property Loop
**Learning:** `components/script/dom/html/htmlformelement.rs`'s `SupportedPropertyNames` method contained an O(N^2) deduplication loop using `names_vec.iter().any(...)`, and unnecessary O(N) heap allocations via `.to_string().is_empty()` during a `retain` operation.
**Action:** Replaced O(N^2) `Vec::any` lookup with an O(1) `HashSet` lookup, achieving O(N) deduplication time. Replaced `.to_string().is_empty()` with native `.is_empty()` on `Atom` to skip heap allocation. Pre-allocated collections using `with_capacity(sourced_names_vec.len())` to prevent dynamic resizing overhead.
