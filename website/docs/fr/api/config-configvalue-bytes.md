---
locale: fr
source_revision: bfcaf340b0c1c1671610ce3a1025921897c59afd9bdf91da4be0b86f5f4d7446
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Bytes"
---
# config::ConfigValue::Bytes

Sélectionner une entrée de configuration Bytes.

```rust
Bytes (& 'a [u8])
```

## Paramètres, résultat et utilisation courante

Emprunte des octets contenant du texte décimal, et non une charge utile décimale binaire. Les modifications du Uint8Array dans le navigateur avant le décodage sont observées ; le décodage ne modifie jamais le tableau. Un UTF-8 incorrect fait échouer l’analyse décimale.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-bytes" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
