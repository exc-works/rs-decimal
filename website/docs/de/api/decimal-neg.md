---
locale: de
source_revision: 16b33be7c90300f9c4b236f60591e3b54c1078c3a213fb54df0f571ca8f60e09
translation_provenance: en-US-german-reviewed
api_id: Decimal::neg
---
# Decimal::neg

Eine unabhängige Dezimalzahl mit umgekehrtem Vorzeichen erzeugen.

```rust
fn neg (& self) -> Self
```

## Parameter, Rückgabe und Hinweise

Liefert Decimal und erhält Skala und nachgestellte Nullen. Null bleibt numerisch null; auch bei Standardeingabe ist das Ergebnis initialisiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-neg" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
