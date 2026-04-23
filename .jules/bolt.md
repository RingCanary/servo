## 2024-05-24 - DOM Property Name Deduplication
**Learning:** `Vec::contains()` and `.iter().any()` are heavily used in DOM collection implementations (`HTMLCollection`, `HTMLFormElement`, `NamedNodeMap`) to deduplicate property names during enumeration, leading to O(N^2) complexity on highly populated collections.
**Action:** Replace linear scans with `std::collections::HashSet` for O(1) membership testing. When deduplicating `Atom` strings (`stylo_atoms`), use `seen.insert(atom.clone())` before performing the heavy `DOMString::from` heap allocation, as cloning an `Atom` is cheap.

## 2024-05-24 - AI Collaboration: Missing Comments
**Learning:** The previous task review flagged my patch because I missed adding the requested O-notation comments explaining the optimization as mandated in the "Always do" rules, and left a scratchpad file (`test_script.sh`) in the working tree.
**Action:** Always re-read the core prompt boundaries ("Always do" and "Never do") explicitly right before running `submit`. Thoroughly review `git status` to ensure zero unintended files remain.
