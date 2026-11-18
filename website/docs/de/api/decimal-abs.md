---
locale: de
source_revision: aa32a5479f7baf77d635cc3726288c9b7c2cfa292167bcd9b91265a058234a8b
translation_provenance: en-US-german-reviewed
api_id: Decimal::abs
---
# Decimal::abs

Eine unabhängige Dezimalzahl mit dem Absolutwert erzeugen.

```rust
fn abs (& self) -> Self
```

## Parameter, Rückgabe und Hinweise

Liefert Decimal und erhält Skala und nachgestellte Nullen. Das Ergebnis ist auch bei Standardeingabe initialisiert; die Eingabe bleibt unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-abs" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
