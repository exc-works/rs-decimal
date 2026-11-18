---
locale: de
source_revision: 19a39638a4734aeba49de6f5af4a9f79e89b842f7b78ba2cf7aeb511f0ebcd6c
translation_provenance: en-US-german-reviewed
api_id: Decimal::cmp
---
# Decimal::cmp

Zwei Zahlen vergleichen und eines von drei Ergebnissen erhalten.

```rust
fn cmp (& self , other : & Self) -> i32
```

## Parameter, Rückgabe und Hinweise

other: &Decimal (Decimal in JavaScript) ausleihen. Liefert i32/number: -1, 0 oder 1. Skala und Initialisierung beeinflussen die numerische Gleichheit nicht. Dies ist die inhärente Methode, nicht Ord::cmp mit dem Rückgabetyp Ordering.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-cmp" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
