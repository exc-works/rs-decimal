---
locale: de
source_revision: 03df94c2d70888cf880122deea0e6ec382870fa1e10b3d6cb2d26b3fc0fb348a
translation_provenance: en-US-german-reviewed
api_id: Decimal::gte
---
# Decimal::gte

Prüfen, ob dieser Wert größer oder gleich einem anderen ist.

```rust
fn gte (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Gleichheit ist numerisch und unabhängig von Skala und Initialisierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-gte" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
