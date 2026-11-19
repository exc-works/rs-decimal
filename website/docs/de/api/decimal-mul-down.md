---
locale: de
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

Dezimalzahlen mit festgelegter Ausgabepräzision multiplizieren.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala ist die größere Operandenskala, nicht deren Summe. mul_exact erhält jede Produktziffer. Down schneidet auch negative Ergebnisse Richtung null ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
