---
locale: en-US
source_revision: e8f2b09a741088cc99921865f54ef434d0d70ac376d4e29b7efc3c00a51c3956
api_id: Decimal::new_from_big_int
---
# Decimal::new_from_big_int

Copy an arbitrary-size integer into an initialized scale-zero Decimal without losing digits.

```rust
fn new_from_big_int (value : & BigInt) -> Self
```

## Parameters and return

Rust borrows `&BigInt`; JavaScript supplies immutable `bigint`. The returned coefficient is independent of the source.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int" locale="en-US" />

## Usage notes and boundaries

Do not convert a JavaScript number to bigint after it has already lost precision; start with an integer literal ending in `n` or exact integer text. The playground accepts at most 4096 input digits; the native library is not subject to this playground policy.

The browser also applies the [resource policy](../resource-policy). JavaScript inputs are validated before calling Rust.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
