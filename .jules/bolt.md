
## 2024-05-19 - Iterator Pre-allocation in ShadowRoot
**Learning:** In Rust, `Vec::from_iter` (via `collect()`) fails to pre-allocate optimally when the iterator is a `FlatMap` or `FilterMap` due to vague size hints from the iterator. This causes unnecessary array resizes and memory allocations.
**Action:** When mapping over items and filtering them (like `Result` filtering), and the upper bound is known (like in `elements_from_point`), replace `.collect()` with `Vec::with_capacity(known_upper_bound)` and iterate using a `for` loop, calling `elements.push()`. This prevents unexpected allocations.
