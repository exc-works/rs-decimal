---
locale: de
source_revision: 59688cddd572412f36dd0f36cc15c34efbc891dab38cd57b614763347d469369
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_nil
---
# Decimal::is_nil

Prüfen, ob eine Dezimalzahl der nicht initialisierte Standardwert ist.

```rust
fn is_nil (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean. Der Standardwert ist nil, konstruierte Null ist initialisiert; beide sind numerisch gleich. Dies ist nicht das Gültigkeitskennzeichen von NullDecimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-nil" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
