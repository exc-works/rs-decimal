---
locale: de
source_revision: 1cd08b3844b9339373458bc008707757ca17e7772da4c888bc88a31f9efef788
translation_provenance: en-US-german-reviewed
api_id: Decimal::min
---
# Decimal::min

Den kleineren von zwei Zahlenwerten kopieren.

```rust
fn min (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei Gleichheit bleiben Skala und Initialisierung des ersten Operanden erhalten. Verwenden Sie in Rust Decimal::min(&left, &right), um nicht die konsumierende Trait-Methode Ord::min auszuwählen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-min" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
