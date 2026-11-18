---
locale: de
source_revision: aa764a04d58bbac840fe64bca919cb70175f064a9010f9a840a1f56ffacabc1c
translation_provenance: en-US-german-reviewed
api_id: Decimal::is_zero
---
# Decimal::is_zero

Vor einer wertabhängigen Operation auf numerische Null prüfen.

```rust
fn is_zero (& self) -> bool
```

## Parameter, Rückgabe und Hinweise

Liefert bool/boolean und auch für den nicht initialisierten Standardwert true. Verwenden Sie is_nil(), wenn die Initialisierung relevant ist.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="basic-is-zero" locale="de" />

## Fehler und Grenzen

Diese Methoden leihen Eingaben aus, ohne sie zu konsumieren oder zu verändern. Die Bibliotheksoperation liefert kein Result. Die Bridge weist freigegebene, verschobene oder fremden Läufen zugehörige Handles ab und wendet die [Ressourcenrichtlinie](../resource-policy) an. Beispiele verpacken skalare Ausgaben in einem `value`-Feld; Decimal-Ausgaben verwenden einen vollständigen Snapshot.

## Verwandte Methoden

[is_nil](./decimal-is-nil) · [equal](./decimal-equal) · [new_from_string](./decimal-new-from-string)
