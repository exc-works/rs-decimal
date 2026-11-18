---
locale: fr
source_revision: 4824e4dc634e3ef762c96aa330a4aec14949a376f5c5d2b586754496c1c0639f
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl Deserialize < 'de >::deserialize"
---
# NullDecimal::impl Deserialize < 'de >::deserialize

Désérialiser avec le visiteur Serde générique.

```rust
fn deserialize < D : Deserializer < 'de > > (d : D) -> Result < Self , D :: Error >
```

## Paramètres, résultat et utilisation courante

deserializer : D implémentant Deserializer ; renvoie Self ou D::Error. Accepte les chaînes décimales exactes, les entrées entières du visiteur et null. Les entrées flottantes du visiteur sont rejetées : un nombre JSON fractionnaire ou avec exposant doit utiliser l’adaptateur JSON brut/config dédié. JSON null produit une valeur nullable invalide avec un zéro interne non initialisé. Les chaînes vides sont invalides. L’adaptateur statique deserialize(source) du navigateur exécute serde_json sur le texte JSON original complet ; il est plus restreint que la prise en charge de désérialiseurs Rust arbitraires. Les erreurs exposent category="serde" et le message original.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-deserialize-de-deserialize" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
