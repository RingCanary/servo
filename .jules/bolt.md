## 2025-10-30 - [Optimize queue shifts in script/dom]
**Learning:** Task queues in `script` component like `runsteps_queues` in `timers.rs`, `pending_pull_intos` in `readablebytestreamcontroller.rs`, and `delayed_tasks` in `document.rs` were using `Vec` and shifting elements via `.remove(0)` which is an O(N) operation.
**Action:** Replace `Vec` with `VecDeque` for queues that frequently use `pop_front` operations to reduce complexity to O(1) and improve queue performance.
