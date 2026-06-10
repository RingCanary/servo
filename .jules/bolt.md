## 2024-05-24 - Zero-Cost Atom Deduplication
**Learning:** While cloning an `Atom` in Servo is cheap (just incrementing an Arc), deduplicating using `HashSet<&Atom>` entirely avoids even this atomic reference-counting overhead, making it a true zero-cost abstraction during iteration.
**Action:** When deduplicating `Atom` sequences (or any interned/ref-counted string), insert references into the `HashSet` (`seen.insert(&elem.name)`) instead of cloning.
