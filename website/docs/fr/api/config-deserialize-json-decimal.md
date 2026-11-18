---
locale: fr
source_revision: a26edb51e2fbe0ae6bac5abd8ce3e12bd13315bb3e836a0d554d5f19af0430ce
translation_provenance: en-US-french-reviewed
api_id: "config::deserialize_json_decimal"
---
# config::deserialize_json_decimal

Désérialiser un champ de configuration JSON exact.

```rust
fn deserialize_json_decimal < 'de , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Decimal , D :: Error >
```

## Paramètres, résultat et utilisation courante

deserializer : D implémentant Serde Deserializer ; destiné à #[serde(deserialize_with = "rs_decimal::config::deserialize_json_decimal")]. Exige une source serde_json et la fonctionnalité config. Conserve les jetons numériques originaux sans aller-retour flottant. Le navigateur fournit un adaptateur concret recevant une chaîne source contenant un scalaire JSON complet, pas un désérialiseur arbitraire. JSON null et les chaînes vides sont rejetés pour la cible non nullable. Les erreurs ont le type d’erreur du désérialiseur Serde ; dans le navigateur, category="serde" conserve leur message au lieu d’inventer des catégories DecimalError. Toute entrée finale autre que des espaces JSON est rejetée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-deserialize-json-decimal" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
