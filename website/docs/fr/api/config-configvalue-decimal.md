---
locale: fr
source_revision: b283786af672117d0c3017e513f97f5df201a379427acab824b383812aa4cf93
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

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
