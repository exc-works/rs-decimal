---
locale: fr
source_revision: 07611a32fa9c50a924b808d879544951b6661718e9f82991f25c28bfa7a925e8
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Signed"
---
# config::ConfigValue::Signed

Sélectionner une entrée de configuration Signed.

```rust
Signed (i128)
```

## Paramètres, résultat et utilisation courante

Transporte un i128 exact signé. Le navigateur exige un bigint dans [-2^127, 2^127-1] ; les number JavaScript et les valeurs hors limites sont rejetés avant conversion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-signed" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
