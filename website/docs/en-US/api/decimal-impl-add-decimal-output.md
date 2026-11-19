---
locale: en-US
source_revision: 7943930854d815d633da7c1bcf125b3fedc2071953907c36e9c6ae78fdf199c7
api_id: "Decimal::impl Add < & Decimal >::Output"
---
# Decimal::impl Add < & Decimal >::Output

Use borrowed Rust Add syntax.

```rust
type Output = Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-output" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
