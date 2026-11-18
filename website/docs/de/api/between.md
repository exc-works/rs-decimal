---
locale: de
source_revision: 4488da41f235921047e8f85b8a68b47bc497bf199529326773333d8f22d38585
translation_provenance: en-US-german-reviewed
api_id: between
---
# between

Prüfen, ob eine Dezimalzahl innerhalb eines einschließlich seiner Grenzen geltenden Zahlenbereichs liegt.

```rust
fn between (v : & Decimal , lower : & Decimal , upper : & Decimal) -> bool
```

## Parameter, Rückgabe und Hinweise

v, lower und upper als &Decimal ausleihen; liefert bool/boolean für lower <= v <= upper. Gleichheit ignoriert die Darstellung. Grenzen werden nicht umgeordnet: lower > upper liefert false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="helper-between" locale="de" />

## Grenzen

Die Bridge erhält Rust-Namen und Parameterreihenfolge. Freie Funktionen stehen im Playground direkt bereit. Ausgeliehene Eingaben werden weder konsumiert noch verändert; ungültige Handles oder solche aus fremden Läufen werden abgewiesen. Alle Operationen folgen der [Ressourcenrichtlinie](../resource-policy).

## Verwandte Methoden

[new_from_string](./decimal-new-from-string) · [is_nil](./decimal-is-nil) · [equal](./decimal-equal)
