## 2025-02-28 - Optimize IFrameCollection Length Check
**Learning:** Iterating over DOM collections that yield ref-counted elements (e.g., `iframes.iter()` mapping to `iframe.element.as_rooted()`) introduces O(N) atomic reference-counting overhead for simple counts; prefer direct `.len()` lookups on the underlying data structures to achieve O(1) performance.
**Action:** When working with collections in the DOM, always check if the underlying data structure provides a fast `len()` method before using iterator counts.

## 2025-02-28 - Build script failure on missing llvm-objdump
**Learning:** `cargo check -p script` and `cargo check -p layout` consistently fail in environments missing `llvm-objdump` (due to `mozjs` dependencies).
**Action:** Rely on `cargo check -p script_traits` or manual logic verification for the `script` crate when `llvm-objdump` is missing.
