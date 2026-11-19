---
locale: de
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
translation_provenance: en-US-german-reviewed
api_id: Decimal::sub
---
# Decimal::sub

Dezimalzahlen exakt subtrahieren.

```rust
fn sub (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
