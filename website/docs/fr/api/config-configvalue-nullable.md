---
locale: fr
source_revision: 5a5d502f0868797711c2e17ee061bb453dff465719822a493b3a33d226cf87b4
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Nullable"
---
# config::ConfigValue::Nullable

Sélectionner une entrée de configuration Nullable.

```rust
Nullable (& 'a NullDecimal)
```

## Paramètres, résultat et utilisation courante

Emprunte une valeur nullable existante. decode_nullable clone la validité et les données masquées ; decode_decimal accepte uniquement les valeurs valides et clone leur Decimal interne. Les modifications effectuées avant le décodage sont observées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-nullable" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
