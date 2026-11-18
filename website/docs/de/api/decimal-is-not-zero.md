---
locale: de
source_revision: aaa15879504e10287ba605474fa0c07c65d1d6607caa2a6ac35b407fe6cd46e4
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_not_zero
---
# Decimal::is_not_zero

Prüfen, ob der Zahlenwert von null verschieden ist.

```rust
fn is_not_zero (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean als Umkehrung von is_zero(). Der nicht initialisierte Standardwert liefert false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-not-zero" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
