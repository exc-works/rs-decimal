---
locale: en-GB
source_revision: 3cc2ce52c4d0ed0d111f2e4df3eea8f2b90cb54656fa97590458dae94eb7b967
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::bit_len
---
# Decimal::bit_len

Count bits of the absolute unscaled coefficient.

```rust
fn bit_len (& self) -> usize
```

## Parameters, return and normal usage

No parameters; Rust returns usize, the bridge returns number. Scale is not applied and the sign bit is not counted: -1.20 has coefficient -120 and bit length 7. Zero, including default zero, returns 0. This is not the highest set-bit index.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-bit-len" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
