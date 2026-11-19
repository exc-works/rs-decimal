---
locale: de
source_revision: adde9dcc9ce010498ad91d2410621453cfaa363aae0ee4c3d4598482bb6eead0
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul
---
# Decimal::mul

Dezimalzahlen mit festgelegter Ausgabepräzision multiplizieren.

```rust
fn mul (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala ist die größere Operandenskala, nicht deren Summe. mul_exact erhält jede Produktziffer. Ein expliziter mode steuert die Rundung; Unnecessary liefert RoundUnnecessary, wenn Ziffern ungleich null verworfen werden müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
