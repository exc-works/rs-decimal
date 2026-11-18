---
locale: de
source_revision: 9a05736a78d12045af2704500028fdc4fdec0490c797ca6d8ba9130b66fbc153
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

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
