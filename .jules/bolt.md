## 2025-03-09 - Vec to VecDeque Migration requires method adjustments
**Learning:** When changing a `Vec` to `VecDeque` for performance reasons (to avoid O(N) shifts on `remove(0)`), any existing `.first()` or `.first_mut()` calls on that collection will fail to compile, because `VecDeque` does not implement `Deref<Target = [T]>` and thus lacks these slice methods.
**Action:** When swapping `Vec` for `VecDeque`, comprehensively grep for and replace `.first()` with `.front()` and `.first_mut()` with `.front_mut()`.
