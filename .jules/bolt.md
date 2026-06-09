## $(date +%Y-%m-%d) - Optimize queues using VecDeque
**Learning:** In `components/script`, task queues such as `OrderingQueues` in `timers.rs`, `pending_pull_intos` in `readablebytestreamcontroller.rs`, and `delayed_tasks` in `document.rs` were using `Vec` and calling `.remove(0)`, resulting in O(N) shifts.
**Action:** Replace `Vec` with `VecDeque` and update methods (e.g., `.remove(0)` to `.pop_front().unwrap()`, `.first()` to `.front()`, `.push()` to `.push_back()`) to achieve O(1) shifts. Ensure that fields using `Default::default()` are inherently compatible with `VecDeque`.
