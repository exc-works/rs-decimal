---
locale: de
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

Konfigurationseingabe Text wählen.

```rust
Text (& 'a str)
```

## Parameter, Rückgabe und übliche Verwendung

Exakten Dezimaltext ausleihen. Äußerer Leerraum wird vom Dezimalkern verarbeitet; äußere SQL-artige Anführungszeichen werden nicht entfernt. Nur exakt leerer Text bedeutet nullable-Abwesenheit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
