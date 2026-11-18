---
locale: en-GB
source_revision: c41b021290595dc4c30480f34c9ca6fcd54ae6ed90ebcef6b793849dbfd2318c
translation_provenance: en-US-reviewed-reuse
api_id: Decimal::float64
---
# Decimal::float64

Convert directly to IEEE binary64 and report exactness.

```rust
fn float64 (& self) -> (f64 , bool)
```

## Parameters, return and normal usage

No parameters; Rust returns (f64, bool), mapped to [number, exact]. Rounds ties to even. The flag means the decimal is represented exactly, not merely that conversion succeeded: 0.1 is inexact. Overflow returns signed infinity with false; underflow can return signed zero. The example records IEEE bits so JSON cannot hide a special value. Playground print also tags NaN, infinities and negative zero with their transported binary64 bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float64" locale="en-GB" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
