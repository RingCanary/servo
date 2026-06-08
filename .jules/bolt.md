## 2024-05-24 - Optimize queue operations from O(N) to O(1) in script component
**Learning:** Found several places where `Vec::remove(0)` was used as a queue pop operation in the script component, leading to O(N) performance overhead. Using `VecDeque` and `pop_front()` changes this to an O(1) operation, improving performance for tight loops where elements are frequently pushed and popped.
**Action:** Replace `Vec` with `VecDeque` in queue-like collections, especially those relying on `.remove(0)` or `.insert(0, item)` in loops.
