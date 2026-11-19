---
locale: de
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: en-US-german-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

Konfigurationseingabe Unsigned wählen.

```rust
Unsigned (u128)
```

## Parameter, Rückgabe und übliche Verwendung

Ein vorzeichenloses u128 exakt transportieren. Der Browser verlangt bigint in [0, 2^128-1], ohne Gleitkommazwischenschritt.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="de" />

## Verwandte Methoden

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
