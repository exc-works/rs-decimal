---
locale: fr
source_revision: 9e6af4fbdd99e4ba366c644aaf4e403edbed7726d41f29fd990845c2a190132b
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Unsigned"
---
# config::ConfigValue::Unsigned

Sélectionner une entrée de configuration Unsigned.

```rust
Unsigned (u128)
```

## Paramètres, résultat et utilisation courante

Transporte un u128 exact non signé. Le navigateur exige un bigint dans [0, 2^128-1], sans intermédiaire flottant.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-unsigned" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
