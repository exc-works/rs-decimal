---
locale: en-US
source_revision: 4edb7e0a9fed4047c94ce74a3f224028a33e2b9c87f41b5e744d6d68a1abd2e7
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

Use borrowed Rust Neg syntax.

```rust
fn neg (self) -> Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. Unary - preserves scale and initializes the result, including default zero. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg on the bridge.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
