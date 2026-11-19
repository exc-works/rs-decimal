---
locale: en-GB
source_revision: 25c9cc1e70c5db48b15f2e6457931c51bc64818138c0422f347742831125e26b
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Add < & Decimal >::add"
---
# Decimal::impl Add < & Decimal >::add

Use borrowed Rust Add syntax.

```rust
fn add (self , other : & Decimal) -> Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-add-decimal-add" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
