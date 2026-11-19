---
locale: en-US
source_revision: c96f97d1831472af0c5d9cbfe01f313b01533a770c9ed531016eff9474a327fd
api_id: "Decimal::impl Sub < & Decimal >::Output"
---
# Decimal::impl Sub < & Decimal >::Output

Use borrowed Rust Sub syntax.

```rust
type Output = Decimal
```

## Normal usage and caveats

Implemented for &Decimal, so operands remain usable. The exact result uses the larger operand scale and is initialized. Associated Output is Decimal. Negative results are allowed; these operators do not apply safe_add/safe_sub nonnegative-result requirements. The trait delegates to the same named core method. Browser JavaScript operators are not overloaded; use add/sub/neg in JavaScript.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="native-decimal-impl-sub-decimal-output" locale="en-US" />

## Execution

This item is native Rust trait syntax; it does not expose a same-named JavaScript operator or SQLx object. The default native documentation harness executes this example.

## Related methods

[Decimal](./decimal) · [scan](./decimal-scan) · [new_from_string](./decimal-new-from-string)
