## 2025-02-12 - HTMLFormElement names duplication optimization
**Learning:** O(N^2) duplication removal in Vec iterators when using `Vec::any` checking, instead of leveraging a fast O(1) HashSet for tracking uniqueness.
**Action:** When filtering unique atoms from a collection and mapping them to DOMStrings, always use an intermediate `HashSet` combined with pre-allocating memory (`Vec::with_capacity`).
