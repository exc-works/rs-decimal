---
locale: de
source_revision: 0d4a74db932ab126e7566fb630352abba8c528142a3d37c615a2694b7fac9c0a
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

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
