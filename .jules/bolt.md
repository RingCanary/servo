## 2024-05-24 - O(1) Dequeue Optimizations
**Learning:** Using `Vec::remove(0)` to process queues leads to O(N) shifts. Several queues in `components/script` (`OrderingQueues`, `pending_pull_intos`, `delayed_tasks`) were using this pattern. Using `VecDeque::pop_front()` provides an O(1) alternative without changing the algorithmic behavior.
**Action:** Always check `.remove(0)` patterns in queues and convert underlying data structures to `VecDeque` where appropriate. Be mindful of updating `first()` and `first_mut()` to `front()` and `front_mut()`.
