---
locale: de
source_revision: 828ccfc55974951d7ced64d269a1959b7a45efe557a7f4202f79dcfc3632ea3f
translation_provenance: en-US-german-reviewed
api_id: Decimal::sign
---
# Decimal::sign

Das Vorzeichen als -1, 0 oder 1 lesen.

```rust
fn sign (& self) -> i32
```

## Parameter, Rückgabe und Hinweise

Liefert i32/number. Initialisierte und nicht initialisierte Null haben Vorzeichen 0; es gibt kein separates negatives Nullvorzeichen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-sign" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
