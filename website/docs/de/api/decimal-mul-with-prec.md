---
locale: de
source_revision: b2363400afff1e4c740938a16816a148faaff0faef10b304c45a776ef2dcc1ee
translation_provenance: en-US-german-reviewed
api_id: Decimal::mul_with_prec
---
# Decimal::mul_with_prec

Dezimalzahlen mit festgelegter Ausgabepräzision multiplizieren.

```rust
fn mul_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal. Die Ergebnisskala beträgt prec Nachkommastellen und muss nichtnegativ sein; negatives prec liefert InvalidPrecision. Ein expliziter mode steuert die Rundung; Unnecessary liefert RoundUnnecessary, wenn Ziffern ungleich null verworfen werden müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-with-prec" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
