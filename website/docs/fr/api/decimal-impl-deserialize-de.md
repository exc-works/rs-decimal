---
locale: fr
source_revision: 290824d81667633f4375180726adda2c851b5415686c46a9a13af6b626baaf4f
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Deserialize < 'de >"
---
# Decimal::impl Deserialize < 'de >

Désérialiser avec le visiteur Serde générique.

```rust
impl< 'de > Deserialize < 'de > for Decimal
```

## Paramètres, résultat et utilisation courante

deserializer : D implémentant Deserializer ; renvoie Self ou D::Error. Accepte les chaînes décimales exactes, les entrées entières du visiteur et null. Les entrées flottantes du visiteur sont rejetées : un nombre JSON fractionnaire ou avec exposant doit utiliser l’adaptateur JSON brut/config dédié. JSON null produit un zéro Decimal non initialisé. Les chaînes vides sont invalides. L’adaptateur statique deserialize(source) du navigateur exécute serde_json sur le texte JSON original complet ; il est plus restreint que la prise en charge de désérialiseurs Rust arbitraires. Les erreurs exposent category="serde" et le message original.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-deserialize-de" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
