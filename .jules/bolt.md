## 2026-04-03 - Optimize ElementsFromPoint allocations
**Learning:** In Rust,  (via `collect()`) fails to pre-allocate correctly when the iterator is a `FlatMap` or `FilterMap` due to vague size hints, forcing repeated reallocations on the heap as elements are yielded.
**Action:** When the upper bounds are known (like downcasting nodes fetched from a query), prefer  and an explicit  loop to build the collection without reallocations.
## 2024-05-24 - Optimize ElementsFromPoint allocations
**Learning:** In Rust, `Vec::from_iter` (via `collect()`) fails to pre-allocate correctly when the iterator is a `FlatMap` or `FilterMap` due to vague size hints, forcing repeated reallocations on the heap as elements are yielded.
**Action:** When the upper bounds are known (like downcasting nodes fetched from a query), prefer `Vec::with_capacity` and an explicit `for` loop to build the collection without reallocations.
