---
locale: de
source_revision: 52eca22e980279ceae03e771dce435ca3ba3fd74042d6f5541b41c478a23c74f
translation_provenance: en-US-german-reviewed
api_id: Decimal::max_ref
---
# Decimal::max_ref

Durch numerischen Vergleich einen ursprünglichen Operanden auswählen.

```rust
fn max_ref < 'a > (& 'a self , other : & 'a Self) -> & 'a Self
```

## Parameter, Rückgabe und übliche Verwendung

other: &Decimal; Rust liefert eine gemeinsame Referenz mit der Lebensdauer der Eingabe. Bei Gleichheit wird self ausgewählt, unter Erhalt von Skala und Initialisierung. Die Bridge liefert den entsprechenden ursprünglichen Wrapper, keine Kopie. Verschieben/Freigeben macht alle JS-Aliase darauf ungültig; JS erzwingt keine Rust-Borrow-Lebensdauern. max/min liefern ein unabhängiges Ergebnis mit eigenem Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-max-ref" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
