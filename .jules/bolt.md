## $(date +%Y-%m-%d) - Optimizing Queue Operations to O(1) in script

**Learning:** When using `Vec` as a queue in Rust, calling `.remove(0)` shifts all subsequent elements, resulting in an O(N) operation. In hot paths (like timer queues or task queues), this can cause significant performance degradation as the queue grows. `VecDeque` is the appropriate data structure for queue-like behaviors where items are frequently pushed to the back and popped from the front.

**Action:** When identifying queues that frequently pop from the front (via `.remove(0)`), refactor the underlying type from `Vec` to `std::collections::VecDeque`. Remember to:
1. Update `Vec::new()` to `VecDeque::new()`.
2. Update `.push()` to `.push_back()`.
3. Update `.first()` and `.first_mut()` to `.front()` and `.front_mut()`.
4. Update `.remove(0)` to `.pop_front().unwrap()`.
5. Ensure `use std::collections::VecDeque;` is imported.
