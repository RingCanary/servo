## 2026-05-28 - VecDeque migration for O(1) popping
**Learning:** Vec shifting via `.remove(0)` introduces hidden O(N) overhead in queues like `pending_pull_intos`, `delayed_tasks`, and `OrderingQueues`. `VecDeque` is a drop-in replacement that retains compatibility with `binary_search_by`/`insert` while offering O(1) `.pop_front()`.
**Action:** Always use `VecDeque` instead of `Vec` when dealing with FIFO queues or arrays requiring `remove(0)`.
