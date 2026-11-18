---
locale: de
source_revision: c63b4582e5ea4e7eac387ce70eb29aed40ab3d4cb5d5c76ae95a6cd8fa1f8f69
translation_provenance: en-US-german-reviewed
api_id: Decimal::scale
---
# Decimal::scale

Die gespeicherte nichtnegative Dezimalskala lesen.

```rust
fn scale (& self) -> usize
```

## Parameter, Rückgabe und Hinweise

Liefert usize als exakte number innerhalb des Skalenbudgets des Playgrounds. Der Wert entspricht precision().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-scale" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
