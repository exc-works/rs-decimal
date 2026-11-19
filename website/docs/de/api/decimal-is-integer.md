---
locale: de
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

Prüfen, ob der Zahlenwert keinen Nachkommaanteil besitzt, also ganzzahlig ist.

```rust
fn is_integer (& self) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert bool/boolean. Geprüft wird der Zahlenwert, nicht eine Skala von null: 12.00 ist eine Ganzzahl. Die Standardnull ist ebenfalls ganzzahlig und hat keinen Bruchteil. Diese Abfragen initialisieren oder verändern die Eingabe nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
