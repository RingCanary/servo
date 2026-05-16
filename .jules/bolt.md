
## 2024-05-24 - [Replace O(N) Vec::remove(0) with O(1) VecDeque for queues]
**Learning:** Found multiple instances of `Vec::remove(0)` in queues (e.g., `OrderingQueues` in `timers.rs`, `delayed_tasks` in `document.rs`, `pending_pull_intos` in `readablebytestreamcontroller.rs`) which results in O(N) shifting of all remaining elements. Replacing the type with `std::collections::VecDeque` eliminates this overhead and achieves O(1) removals.
**Action:** Always favor `VecDeque` when queues perform frequent `pop_front` (or equivalent `.remove(0)`) operations.
