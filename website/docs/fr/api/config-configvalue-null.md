---
locale: fr
source_revision: 78ca92acc2a8523b4e668da7c55a57a56b16410b6455b88e11d916edee01152c
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

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
