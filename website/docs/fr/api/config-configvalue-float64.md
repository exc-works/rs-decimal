---
locale: fr
source_revision: 032d932903a81f0f03e2bde5c0ffb35c8a1064335f6ef0a3149d91077fa81075
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Sélectionner une entrée de configuration Float64.

```rust
Float64 (f64)
```

## Paramètres, résultat et utilisation courante

Transporte un binary64 via la conversion décimale la plus courte du cœur. Les valeurs non finies renvoient Unmarshal ; les chiffres déjà perdus par l’arrondi flottant ne peuvent pas être récupérés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
