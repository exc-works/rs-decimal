---
locale: en-US
source_revision: 42bb4c692d19b0113bb004bfa29bdbc283b0bb157f90295cf679a7adf7e3388f
api_id: "Decimal::impl Neg"
---
# Decimal::impl Neg

Use borrowed Rust Neg syntax.

```rust
impl Neg for & Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. Unary - preserves scale and initializes the result, including default zero. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
