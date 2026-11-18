---
locale: de
source_revision: 775716427f3737c65dee2b46317125f81d53dce3779b9de7255849e290c2308b
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_positive
---
# Decimal::is_positive

Prüfen, ob der Zahlenwert strikt positiv ist.

```rust
fn is_positive (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean. Null einschließlich des nicht initialisierten Standardwerts liefert false.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-positive" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
