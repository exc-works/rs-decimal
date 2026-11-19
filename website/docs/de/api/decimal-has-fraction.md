---
locale: de
source_revision: e9defed137146374a74cf0b0a14760988cf4cd8ad97babd7fd11a73ccc7037b3
translation_provenance: en-US-german-reviewed
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Prüfen, ob der Zahlenwert einen Nachkommaanteil besitzt.

```rust
fn has_fraction (& self) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert bool/boolean. Geprüft wird der Zahlenwert, nicht eine Skala von null: 12.00 ist eine Ganzzahl. Die Standardnull ist ebenfalls ganzzahlig und hat keinen Bruchteil. Diese Abfragen initialisieren oder verändern die Eingabe nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
