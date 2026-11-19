---
locale: de
source_revision: 1edc6ba7e191babd98a0717864d8696548b2086ee97162b567ea1d847a357dad
translation_provenance: en-US-german-reviewed
api_id: Decimal::add
---
# Decimal::add

Dezimalzahlen exakt addieren.

```rust
fn add (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; liefert einen neuen Decimal mit der größeren Operandenskala. Die Ausrichtung ist exakt und verwirft keine Ziffern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
