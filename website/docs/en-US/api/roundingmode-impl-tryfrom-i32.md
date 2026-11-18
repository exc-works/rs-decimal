---
locale: en-US
source_revision: e38006128b404f87e5b765e2fa64c4d3f058b23c6a114c6e9a68f4ed1634f91f
api_id: "RoundingMode::impl TryFrom < i32 >"
---
# RoundingMode::impl TryFrom < i32 >

Validate a raw rounding discriminant with TryFrom.

```rust
impl TryFrom < i32 > for RoundingMode 
```

## Normal usage and caveats

value: i32; returns `Result<RoundingMode, DecimalError>`. Codes 0–6 use the pinned mode mapping, including 5=HalfEven. Unknown codes return InvalidArgument without panicking. The associated Error type is DecimalError. This delegates to from_raw; browser code uses RoundingMode.from_raw(number), with strict i32 parameter checks.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
