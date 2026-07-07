## 2025-10-30 - [Optimize Queue Operations in `components/script`]
**Learning:** Task queues (like `delayed_tasks` in `document.rs`, `pending_pull_intos` in `readablebytestreamcontroller.rs`, and `OrderingQueues` in `timers.rs`) frequently used `Vec::remove(0)`, causing O(N) shifts.
**Action:** Replace `Vec` with `VecDeque` for queues to achieve O(1) `pop_front` operations, updating insertion logic to use `push_back` and element access to `front`/`front_mut`.
