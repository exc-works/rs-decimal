---
locale: de
source_revision: f49d2dc4c074cf4f52b2a3a197eb1d13f1144942ab8a1df7ea4bf51caaaa350f
translation_provenance: en-US-german-reviewed
api_id: max
---
# max

Die größere von zwei Dezimalzahlen mit einer freien Funktion kopieren.

```rust
fn max (a : & Decimal , b : & Decimal) -> Decimal
```

## Parameter, Rückgabe und Hinweise

a und b als &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei numerischer Gleichheit bleiben Skala und Initialisierung von a erhalten. Es gilt dieselbe Bibliothekssemantik wie bei Decimal::max.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-max" locale="de" />

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
