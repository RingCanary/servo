## 2024-05-23 - DocumentTree Allocation Churn
**Learning:** The `DocumentTree` is reconstructed on every call to `documents_in_order`, which is likely called during rendering updates. This involves allocating a new HashMap and a new Vec every time.
**Action:** Pre-allocate collections using `with_capacity` based on the known number of documents (`DocumentCollection::len()`) to minimize reallocation overhead.
