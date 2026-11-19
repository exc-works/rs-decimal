---
locale: de
source_revision: c991ff009b87a451c4d3a01a7e163c4b8e995107fc18ca0a0ef4ec120e3aee71
translation_provenance: en-US-german-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

Durch numerischen Vergleich einen ursprünglichen Operanden auswählen.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert eine gemeinsame Referenz mit der Lebensdauer der Eingabe. Bei Gleichheit wird self ausgewählt, unter Erhalt von Skala und Initialisierung. Die JavaScript-API liefert den entsprechenden ursprünglichen Wrapper, keine Kopie. Verschieben/Freigeben macht alle JS-Aliase darauf ungültig; JS erzwingt keine Rust-Borrow-Lebensdauern. max/min liefern ein unabhängiges Ergebnis mit eigenem Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
