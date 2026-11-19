---
locale: fr
source_revision: 8d101e011ba2e4f441ebfc6371c73cb22589c6224599cc87e367af949b20dc8f
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Decimal"
---
# config::ConfigValue::Decimal

Sélectionner une entrée de configuration Decimal.

```rust
Decimal (& 'a Decimal)
```

## Paramètres, résultat et utilisation courante

Emprunte un décimal existant et clone son coefficient, son échelle et son état d’initialisation lors du décodage. La source reste utilisable. Libérer ou déplacer la source dans le navigateur avant le décodage invalide la variante empruntée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-decimal" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
