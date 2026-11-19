---
locale: en-US
source_revision: cc346dbc73e3d203c64f61c0e3e68ec08682dec8479c74ee4ea48b4441aef5c2
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

Do not convert a JavaScript number to bigint after it has already lost precision; start with an integer literal ending in `n` or exact integer text. The playground accepts at most 4096 input digits; the native library is not subject to this playground policy.

## Related methods

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
