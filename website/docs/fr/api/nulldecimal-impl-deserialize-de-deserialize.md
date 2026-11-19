---
locale: fr
source_revision: c31182c59d69a63bcf444ed429e3135dcff513f7463525a159b980c0e11afd3d
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

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
