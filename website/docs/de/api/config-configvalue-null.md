---
locale: de
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

Konfigurationseingabe Null wählen.

```rust
Null
```

## Parameter, Rückgabe und übliche Verwendung

Eine fehlende Konfigurationseingabe. decode_nullable liefert einen ungültigen Wert; decode_decimal weist sie mit Unmarshal ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
