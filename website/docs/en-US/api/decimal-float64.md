---
locale: en-US
source_revision: 336fb83395651877f8e8be84274d454b59233c0e0829f7aed38b4d6aa19fe60d
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

<MethodExample id="conversion-decimal-float64" locale="en-US" />

## Related methods

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
