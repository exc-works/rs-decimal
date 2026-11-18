---
locale: en-GB
source_revision: 5c989601e532ba1185143daec0df2f7cc1f95dc1823993238378295a66882fcf
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Sub < & Decimal >"
---
# Decimal::impl Sub < & Decimal >

Use borrowed Rust Sub syntax.

```rust
impl Sub < & Decimal > for & Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg on the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
