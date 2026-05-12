## 2024-11-20 - [VecDeque Drop-in Replacement for O(1) Shifts]
**Learning:** When swapping `Vec` for `std::collections::VecDeque` to eliminate O(N) `.remove(0)` shifts, standard methods like `binary_search_by` and `insert(idx)` continue to work identically, and struct initialization using `Default::default()` automatically adapts without needing to explicitly call `VecDeque::new()`.
**Action:** Always prefer `VecDeque` over `Vec` for queues where elements are popped from the front, and leverage `Default::default()` for cleaner struct initialization.
