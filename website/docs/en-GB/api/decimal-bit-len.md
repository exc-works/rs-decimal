---
locale: en-GB
source_revision: ce7339c2b8115fd42455e27eb624e03a51de3a069de71c01158ea752ddf6833a
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

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
