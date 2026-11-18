---
locale: en-US
source_revision: 821c4c23615da2c03d94630c3843c11502e27fc2644faac01dc3a81234b52029
api_id: Decimal::float32
---
# Decimal::float32

Convert directly to IEEE binary32 and report exactness.

```rust
fn float32 (& self) -> (f32 , bool)
```

## Parameters, return and normal usage

No parameters; Rust returns (f32, bool), mapped to [number, exact]. Rounds ties to even. The flag means the decimal is represented exactly, not merely that conversion succeeded: 0.1 is inexact. Overflow returns signed infinity with false; underflow can return signed zero. Conversion rounds directly to binary32 rather than first rounding to binary64. The example records IEEE bits so JSON cannot hide a special value. Playground print also tags NaN, infinities and negative zero with their transported binary64 bits.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-float32" locale="en-US" />

## Browser boundaries

Integer transport does not pass through lossy JS numbers. Conversion tuples retain Rust value/flag order; false does not promise an exact result. Borrowed inputs remain unchanged and constructors initialize results. The [resource policy](../resource-policy) bounds integer inputs, output scale and rational coefficient growth separately from library errors. Rust import syntax is explicitly labeled as native-tested.

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
