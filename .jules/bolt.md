## 2024-06-27 - [Avoid to_string().is_empty() and O(N^2) deductions]
**Learning:** Checking string emptiness by allocating with `.to_string().is_empty()` is wasteful when `.is_empty()` is natively available. Also, finding uniqueness in a Vec using `Vec::contains()` or `.iter().any()` inside a loop creates O(N^2) complexity.
**Action:** Use `.is_empty()` directly on Atoms where possible, and use `HashSet::with_capacity` combined with `.insert()` to filter duplicates in O(N) time when converting vectors.
