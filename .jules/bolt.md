## 2025-03-01 - DOMString Deduplication Optimization
**Learning:** `Atom` references inside DOM Collections (like `HTMLCollection` and `HTMLFormElement`) are frequently interned strings. Checking `DOMString::from()` repeatedly inside an $O(N^2)$ `.contains()` loop causes significant reallocation overhead and CPU cycles.
**Action:** When filtering or deduplicating string values based on `Atom` properties (like ID or name), use `std::collections::HashSet` with `Atom::clone()` (which is very cheap) to achieve an $O(N)$ pass, and only allocate the `DOMString::from()` wrapper for unique items.
