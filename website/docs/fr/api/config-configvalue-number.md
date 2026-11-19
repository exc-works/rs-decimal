---
locale: fr
source_revision: d9b1450ea64f3f424b8c3303a77d21fe7ac3c0fd6d9a478841f3ae7df9627834
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Number"
---
# config::ConfigValue::Number

Sélectionner une entrée de configuration Number.

```rust
Number (& 'a str)
```

## Paramètres, résultat et utilisation courante

Transporte le texte numérique source sans conversion flottante. Le cœur analyse la syntaxe décimale ; cette variante ne valide pas toute la grammaire JSON. Un Number vide est invalide même pour une cible nullable.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-number" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
