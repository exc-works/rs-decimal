---
locale: de
source_revision: 633e4a423f3366a4dc502e8d35d4ffa6a3d6f7dd5647951875fd3e31e57bf2c5
translation_provenance: en-US-german-reviewed
api_id: Decimal::lte
---
# Decimal::lte

Prüfen, ob dieser Wert kleiner oder gleich einem anderen ist.

```rust
fn lte (& self , other : & Self) -> bool
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert bool/boolean. Gleichheit ist numerisch und unabhängig von Skala und Initialisierung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-lte" locale="de" />

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
