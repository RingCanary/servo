## 2024-05-24 - HTMLFormElement supported property names O(N^2) complexity

**Learning:** When retrieving `supported_property_names` in `components/script/dom/html/htmlformelement.rs`, deduplicating elements by checking `!names_vec.iter().any(...)` causes O(N^2) complexity because `names_vec` is scanned for every inserted element.

**Action:** Use a `std::collections::HashSet` pre-allocated with `with_capacity(sourced_names_vec.len())` to perform O(1) membership checks to track seen names while building the final output vector, reducing the complexity to O(N).

## 2024-05-24 - Unnecessary String allocations in HTMLFormElement name checking

**Learning:** Checking if an `Atom` (or string-like type) is empty by calling `.to_string().is_empty()` unnecessarily allocates memory to build the temporary string representation before immediately discarding it.

**Action:** Directly call the `.is_empty()` method on the underlying type whenever available to skip the allocation.
