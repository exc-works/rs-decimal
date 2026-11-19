---
locale: de
source_revision: 1e9f449a346e8641c2e3739002dd283333ecd696e411549249e2395dfdddd2f0
translation_provenance: en-US-german-reviewed
api_id: Decimal::min_ref
---
# Decimal::min_ref

Durch numerischen Vergleich einen ursprünglichen Operanden auswählen.

```rust
fn min_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert eine gemeinsame Referenz mit der Lebensdauer der Eingabe. Bei Gleichheit wird self ausgewählt, unter Erhalt von Skala und Initialisierung. Die Bridge liefert den entsprechenden ursprünglichen Wrapper, keine Kopie. Verschieben/Freigeben macht alle JS-Aliase darauf ungültig; JS erzwingt keine Rust-Borrow-Lebensdauern. max/min liefern ein unabhängiges Ergebnis mit eigenem Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-min-ref" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
