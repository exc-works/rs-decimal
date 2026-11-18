---
locale: de
source_revision: b70b6d632eef5a013491ea47ed27e9d8eea36fee70b75cd0db8637b8b6a6ccc5
translation_provenance: en-US-german-reviewed
api_id: Decimal::coefficient
---
# Decimal::coefficient

Den exakten unskalierten Ganzzahlkoeffizienten untersuchen.

```rust
fn coefficient (& self) -> & BigInt
```

## Parameter, Rückgabe und Hinweise

Rust leiht &BigInt aus; die Bridge liefert einen unveränderlichen bigint-Snapshot, keine live aktualisierte Referenz. Der Koeffizient enthält nachgestellte Nullen und ist nicht der Ganzzahlteil der Dezimalzahl. Das Beispiel gibt bigint als Text aus, damit JSON alle Ziffern erhält.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-coefficient" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
