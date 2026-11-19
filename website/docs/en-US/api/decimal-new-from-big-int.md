---
locale: en-US
source_revision: 41da382d7882f55308ed631f052fb5ac78093f335752392d35d023f744890035
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

## Usage notes

Do not convert a JavaScript number to bigint after it has already lost precision; start with an integer literal ending in `n` or exact integer text.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
