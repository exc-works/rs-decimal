---
locale: fr
source_revision: 89abd084e8bb8541133194e2046ba4621ebe68fb1c342334ebc3bf5d1ca7dcec
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Text"
---
# config::ConfigValue::Text

Sélectionner une entrée de configuration Text.

```rust
Text (& 'a str)
```

## Paramètres, résultat et utilisation courante

Emprunte du texte décimal exact. Le cœur traite les espaces aux extrémités ; les guillemets externes de style SQL ne sont pas supprimés. Seul un texte exactement vide représente l’absence pour une cible nullable.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-text" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
