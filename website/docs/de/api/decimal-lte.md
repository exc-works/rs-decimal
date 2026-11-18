---
locale: de
source_revision: 60ca5295cf77ea0e6cccdadb5d937c5adbdd69051a5dbac42387fec9ee90828c
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

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
