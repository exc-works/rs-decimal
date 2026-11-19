---
locale: en-GB
source_revision: ae30eee52b8243c53d2363e82db8a8847f306fed0233c1fadf3d83f102fe5452
translation_provenance: en-US-reviewed-reuse
api_id: "Decimal::impl Neg::neg"
---
# Decimal::impl Neg::neg

Use borrowed Rust Neg syntax.

```rust
fn neg (self) -> Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. Unary - preserves scale and initializes the result, including default zero. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-neg-neg" locale="en-GB" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
