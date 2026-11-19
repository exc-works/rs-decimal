---
locale: en-GB
source_revision: c7bf1c667592f8239154dc21841d27e0f4e94230da52fdab553a96aeaa5e48bf
translation_provenance: en-US-reviewed-reuse
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

<MethodExample id="conversion-decimal-float32" locale="en-GB" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
