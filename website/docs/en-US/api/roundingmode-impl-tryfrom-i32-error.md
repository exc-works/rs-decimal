---
locale: en-US
source_revision: f397dc1220fc731959fe63aa3e5a1c46b0d68942bea1f3f5f567773d91995a1b
api_id: "RoundingMode::impl TryFrom < i32 >::Error"
---
# RoundingMode::impl TryFrom < i32 >::Error

Validate a raw rounding discriminant with TryFrom.

```rust
type Error = DecimalError
```

## Normal usage and caveats

value: i32; returns `Result<RoundingMode, DecimalError>`. Codes 0–6 use the pinned mode mapping, including 5=HalfEven. Unknown codes return InvalidArgument without panicking. The associated Error type is DecimalError. This delegates to from_raw; browser code uses RoundingMode.from_raw(number), with strict i32 parameter checks.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-roundingmode-impl-tryfrom-i32-error" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
