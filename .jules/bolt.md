## 2024-05-24 - Pre-allocating Vecs over `.flat_map().collect()`
**Learning:** In Rust, `Vec::from_iter` (via `collect()`) fails to pre-allocate correctly when the iterator is a `FlatMap` or `FilterMap` due to vague size hints. This can cause unnecessary reallocations in hot paths like `elements_from_point` queries from the layout system.
**Action:** Always prefer manually iterating with a pre-allocated `Vec::with_capacity(expected_len)` over `.flat_map(...).collect()` or `.filter_map(...).collect()` when the exact or upper bound length is known, especially in DOM API paths returning `Vec`.
