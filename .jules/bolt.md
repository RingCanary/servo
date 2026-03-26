## 2024-03-26 - [IFrameCollection Length Optimization]
**Learning:** Found an `O(N)` performance bottleneck where `.iter().count()` was used to count items in `IFrameCollection`, when `IFrameCollection` wraps a `Vec` and could easily expose a `len()` method for `O(1)` counting.
**Action:** Always check if a collection wrapping a basic type like `Vec` can expose `len()` instead of relying on iterators.

## 2024-03-26 - [DOMString::from(&*atom) allocation and uniqueness check optimization]
**Learning:** Found an O(N^2) complexity issue combined with unnecessary allocations in `HTMLCollection::SupportedPropertyNames`. The function iterates over elements, converts `Atom` IDs/names to `DOMString` (which allocates), and then uses `Vec::contains` (O(N) check) to ensure uniqueness before pushing.
**Action:** When filtering unique items represented as `Atom`s, use a `HashSet<Atom>` for O(1) uniqueness checks. `Atom` cloning is cheap. Defer the expensive conversion to `DOMString` until after the uniqueness check confirms it's a new item.
