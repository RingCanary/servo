## 2025-02-28 - Optimize SupportedPropertyNames O(N^2) complexity with HashSet

**Learning:** Iterating over DOM collections to collect unique properties (e.g., `SupportedPropertyNames` in `HTMLCollection` and `HTMLFormElement`) was using `Vec::contains` inside a loop, resulting in O(N^2) complexity where N is the number of elements with matching properties. Converting `Atom` to `DOMString` and pushing conditionally is less efficient than using a `HashSet<Atom>` for uniqueness tracking.
**Action:** When deduplicating strings or atoms in a loop, pre-allocate a `HashSet` and use `seen.insert(atom.clone())`. Then conditionally convert to `DOMString` and push to the result `Vec`. This reduces time complexity from O(N^2) to O(N).
