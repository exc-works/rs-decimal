---
locale: de
source_revision: aed80a094ad154f3abfbbdc72125d175f579082056ef3b224d8dda331e64cff5
translation_provenance: en-US-german-reviewed
api_id: Decimal::into_rescaled
---
# Decimal::into_rescaled

Eine Dezimalzahl beim Ändern ihrer Nachkommastellen konsumieren.

```rust
fn into_rescaled (self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize; mode: RoundingMode. Wie rescale rundet dies oder hängt Nullen an und initialisiert das Ergebnis stets. Der Empfänger wird auch bei InvalidPrecision, RoundUnnecessary oder einem Browser-Ressourcenfehler aus Rust verschoben. Ungültige JS-Typen/Bereiche werden vor Rust abgewiesen und konsumieren ihn nicht.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-into-rescaled" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
