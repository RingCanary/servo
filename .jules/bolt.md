## 2025-02-28 - [HTMLFormElement Iteration Overhead]
**Learning:** Checking for string emptiness by allocating an entire string via `Atom::to_string().is_empty()` and verifying presence in a growing `Vec` using `iter().any()` causes an O(N^2) time complexity with unnecessary heap allocations in DOM collection property queries.
**Action:** Always use the native `.is_empty()` provided on interned string wrappers like `Atom` and switch to using a `HashSet` pre-allocated with `with_capacity` when deduplicating elements to achieve O(N) deduplication time.
