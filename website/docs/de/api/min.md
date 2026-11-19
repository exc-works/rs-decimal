---
locale: de
source_revision: 9f58b0f5c6b478fabd100e00297f6dfcdae4ff2a32ead701d8b208c279651c42
translation_provenance: en-US-german-reviewed
api_id: min
---
# min

Die kleinere von zwei Dezimalzahlen mit einer freien Funktion kopieren.

```rust
fn min (a : & Decimal , b : & Decimal) -> Decimal
```

## Parameter, Rückgabe und Hinweise

a und b als &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei numerischer Gleichheit bleiben Skala und Initialisierung von a erhalten. Es gilt dieselbe Bibliothekssemantik wie bei Decimal::min.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-min" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
