---
locale: fr
source_revision: 846797abc20f334aac55a6cc16f18858d3e8f7870cd49af2d7f236e4e0d093b3
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Null"
---
# config::ConfigValue::Null

Sélectionner une entrée de configuration Null.

```rust
Null
```

## Paramètres, résultat et utilisation courante

Une entrée de configuration absente. decode_nullable renvoie une valeur invalide ; decode_decimal la rejette avec Unmarshal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-null" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
