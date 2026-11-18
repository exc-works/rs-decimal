---
locale: de
source_revision: 075546f79884d636c4e552add1629511a5f2dca740186a5700262f084c5bd8a5
translation_provenance: en-US-german-reviewed
api_id: hundred
---
# hundred

Die Dezimalganzzahl einhundert konstruieren.

```rust
fn hundred () -> Decimal
```

## Parameter, Rückgabe und Hinweise

Keine Parameter; liefert einen neuen initialisierten Decimal mit Skala null. Dies ist ein Wertkonstruktor, keine gemeinsam veränderliche Konstante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-hundred" locale="de" />

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
