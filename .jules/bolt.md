## 2024-05-22 - [DevTools Performance Optimization]
**Learning:** `CSSStyleDeclaration` inspection in DevTools involved excessive string allocations and lock contention due to property name round-trips (Index -> String -> PropertyId -> Value).
**Action:** Implemented `inspect_style_at` to access property data directly by index, bypassing redundant parsing and locking.

## 2024-05-22 - [Build Environment Limitations]
**Learning:** `cargo check -p script` and `cargo test` fail due to missing `llvm-objdump` required by `mozjs_sys` build script.
**Action:** Rely on careful code review and partial verification for `script` component changes until the environment is fixed.
