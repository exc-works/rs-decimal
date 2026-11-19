---
locale: en-GB
source_revision: 4de2b7f049c588d3fb290c5d299ab4e7f3436be10c0a26243bffab3f3e0229cf
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Neg::Output"
---
# Decimal::impl Neg::Output

Use borrowed Rust Neg syntax.

```rust
type Output = Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. Unary - preserves scale and initializes the result, including default zero. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-output" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
