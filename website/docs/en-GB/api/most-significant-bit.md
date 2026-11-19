---
locale: en-GB
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
translation_provenance: en-US-reviewed-reuse
api_id: most_significant_bit
---
# most_significant_bit

Find the zero-based highest set-bit index.

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## Parameters, return and normal usage

value: &BigInt, represented by bigint. Returns Result<usize, DecimalError>/number. Zero and one both return 0; 120 returns 6. Negative input returns InvalidArgument. This is a free function, not a Decimal method; pass coefficient() when that is the intended integer.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
