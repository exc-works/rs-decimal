---
locale: en-GB
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
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

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
