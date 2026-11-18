---
locale: en-US
source_revision: 37cc4be0db267eeddae241a5b2cd03b92e50197f5cac7b3019d106b1e9e0c179
api_id: "Decimal::impl Add < & Decimal >"
---
# Decimal::impl Add < & Decimal >

Use borrowed Rust Add syntax.

```rust
impl Add < & Decimal > for & Decimal 
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg on the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
