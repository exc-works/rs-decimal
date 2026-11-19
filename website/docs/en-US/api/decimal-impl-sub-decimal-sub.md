---
locale: en-US
source_revision: c4a0a7eefa609b0c57e86546936b34ea64a5ece51d5f57a28f19151facde5f69
api_id: "Decimal::impl Sub < & Decimal >::sub"
---
# Decimal::impl Sub < & Decimal >::sub

Use borrowed Rust Sub syntax.

```rust
fn sub (self , other : & Decimal) -> Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-sub" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
