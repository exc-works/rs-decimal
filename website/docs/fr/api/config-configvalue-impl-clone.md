---
locale: fr
source_revision: e777d7a0d2eada54e45b9094274df294b6fb9f6fd490746dfc2797aa8d3acb40
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::impl Clone"
---
# config::ConfigValue::impl Clone

Utiliser l’implémentation native de Clone.

```rust
impl Clone for config::ConfigValue
```

## Paramètres, résultat et utilisation courante

ConfigValue est une petite énumération typée. Clone/Copy conservent les références empruntées au lieu de copier profondément leur cible. Debug fournit du texte de diagnostic, pas un format de transmission stable. La syntaxe des traits Rust natifs est testée séparément et ne constitue pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-impl-clone" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
