---
locale: de
source_revision: 5956431328357316a87f05c2d1543bb100b12923bb2d50b1e21ff3f96212db45
translation_provenance: en-US-german-reviewed
api_id: Decimal::max
---
# Decimal::max

Den größeren von zwei Zahlenwerten kopieren.

```rust
fn max (& self , other : & Self) -> Self
```

## Parameter, Rückgabe und Hinweise

other: &Decimal ausleihen; liefert einen unabhängigen Decimal. Bei Gleichheit bleiben Skala und Initialisierung des ersten Operanden erhalten. In Rust wählt Decimal::max(&left, &right) diese ausleihende inhärente Methode explizit; Ord::max übernimmt den Besitz.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-max" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
