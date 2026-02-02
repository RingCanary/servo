## 2025-02-18 - [Allocating Vectors]
**Learning:** `Vec::from_iter` (via `collect()`) often fails to pre-allocate correctly when the iterator is a `FlatMap` or `FilterMap` because the size hint is `(0, Some(upper_bound))`. This leads to `O(log n)` reallocations.
**Action:** When the upper bound is known (e.g. source is a `Vec`), use `Vec::with_capacity(len)` and `extend()` or a manual loop to guarantee `O(1)` allocation.
