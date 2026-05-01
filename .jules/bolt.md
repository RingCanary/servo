## 2026-05-01 - Replace Vec::remove(0) with VecDeque::pop_front()
**Learning:** Found an anti-pattern where vectors were used as queues using `remove(0)`, leading to O(N) element shifts on pop.
**Action:** When a queue needs FIFO ordering, replace `Vec` with `VecDeque` so `.pop_front()` becomes an O(1) operation.
