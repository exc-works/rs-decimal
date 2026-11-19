---
locale: de
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

Konfigurationseingabe Signed wählen.

```rust
Signed (i128)
```

## Parameter, Rückgabe und übliche Verwendung

Ein vorzeichenbehaftetes i128 exakt transportieren. Der Browser verlangt bigint in [-2^127, 2^127-1]; JS-Zahlen und Werte außerhalb des Bereichs werden vor Konvertierung abgewiesen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
