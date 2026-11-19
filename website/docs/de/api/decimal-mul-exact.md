---
locale: de
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

Dezimalzahlen exakt multiplizieren.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala ist die Summe der Operandenskalen; jede Produktziffer bleibt erhalten. Skalenüberlauf im Kern liefert Overflow. Es gibt keinen Rundungsmodusparameter.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
