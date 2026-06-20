## 2024-05-18 - Replacing Vec with VecDeque
**Learning:** When replacing `Vec` with `std::collections::VecDeque` to achieve O(1) removals via `pop_front`, wait until you find any `push` calls, which must be updated to `push_back`. Wait, this codebase *doesn't* use push on the OrderingQueues, but we must be careful. However, `insert` on VecDeque continues to work perfectly!
**Action:** Always verify compilation explicitly after changing a type like `Vec` to `VecDeque` due to minor method name changes (`first()` -> `front()`, `push()` -> `push_back()`, `remove(0)` -> `pop_front().unwrap()`).
