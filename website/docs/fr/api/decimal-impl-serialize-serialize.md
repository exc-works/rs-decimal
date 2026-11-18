---
locale: fr
source_revision: cc361ad62f1fb6d191828c27bb20bc087e8e30dee2f27936c25d2179d5adc40a
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Serialize::serialize"
---
# Decimal::impl Serialize::serialize

Sérialiser avec le trait Serde de Rust.

```rust
fn serialize < S : Serializer > (& self , s : S) -> Result < S :: Ok , S :: Error >
```

## Paramètres, résultat et utilisation courante

serializer : S implémentant Serializer ; renvoie S::Ok/S::Error. Le cœur sérialise des chaînes décimales canoniques ou null. Un Decimal non initialisé est sérialisé en null ; un zéro initialisé est sérialisé en chaîne "0". Les zéros fractionnaires finaux sont supprimés. L’adaptateur serialize() du navigateur choisit serde_json et renvoie le texte JSON complet. Il n’expose pas de sérialiseurs Rust arbitraires.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize-serialize" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
