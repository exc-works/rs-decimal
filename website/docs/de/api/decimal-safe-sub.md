---
locale: de
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: en-US-german-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

Dezimalzahlen exakt subtrahieren und ein nichtnegatives Ergebnis verlangen.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern. Rust liefert Result: Nur das Ergebnis muss nichtnegativ sein, nicht jeder Operand. Ein negatives Ergebnis liefert InvalidArgument; null ist erlaubt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
