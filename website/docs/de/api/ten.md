---
locale: de
source_revision: df5bdc8fba1feff0933958fdceab2cd0bf8d1a541a0f2e47f17bffe6a6338d47
translation_provenance: en-US-german-reviewed
api_id: ten
---
# ten

Die Dezimalganzzahl zehn konstruieren.

```rust
fn ten () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen initialisierten Decimal mit Skala null. Verwenden Sie rescale, wenn Nachkommastellen benötigt werden.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-ten" locale="de" />

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
