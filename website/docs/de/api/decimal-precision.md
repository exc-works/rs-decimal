---
locale: de
source_revision: c8a99895a920cbd49f4dba891779055021c167a0addb6791872293807a1937f8
translation_provenance: en-US-german-reviewed
api_id: Decimal::precision
---
# Decimal::precision

Die gespeicherte Anzahl der Nachkommastellen lesen.

```rust
fn precision (& self) -> isize
```

## Parameter, Rückgabe und Hinweise

Liefert isize (eine ganzzahlige number im Browser). Dies ist die Skala, nicht die Anzahl signifikanter Ziffern. Nachgestellte Nullen zählen mit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-precision" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
