---
locale: en-US
source_revision: cd78c2dcf4b67b04ef802fd73ba3ef0da02aad24d1bb17be5cc5df5d221df3cd
api_id: Decimal::bit_len
---
# Decimal::bit_len

Count bits of the absolute unscaled coefficient.

```rust
fn bit_len (& self) -> usize
```

## Parameters, return and normal usage

No parameters; Rust returns usize, JavaScript returns number. Scale is not applied and the sign bit is not counted: -1.20 has coefficient -120 and bit length 7. Zero, including default zero, returns 0. This is not the highest set-bit index.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="en-US" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
