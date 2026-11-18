---
locale: en-GB
source_revision: 584a930ae7d128f9ab20d43eb7ffff2b455374bd06ee0e33745542254e1599b0
translation_provenance: en-US-reviewed-reuse
api_id: "RoundingMode::impl TryFrom < i32 >::try_from"
---
# RoundingMode::impl TryFrom < i32 >::try_from

Validate a raw rounding discriminant with TryFrom.

```rust
fn try_from (value : i32) -> Result < Self , Self :: Error >
```

## Normal usage and caveats

value: i32; returns `Result<RoundingMode, DecimalError>`. Codes 0–6 use the pinned mode mapping, including 5=HalfEven. Unknown codes return InvalidArgument without panicking. The associated Error type is DecimalError. This delegates to from_raw; browser code uses RoundingMode.from_raw(number), with strict i32 parameter checks.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-try-from" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
