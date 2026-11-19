---
locale: de
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Konfigurationseingabe Float64 wählen.

```rust
Float64 (f64)
```

## Parameter, Rückgabe und übliche Verwendung

binary64 über die kürzeste Dezimalkonvertierung des Kerns transportieren. Nicht endliche Werte liefern Unmarshal; durch Gleitkommarundung bereits verlorene Ziffern sind nicht wiederherstellbar.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
