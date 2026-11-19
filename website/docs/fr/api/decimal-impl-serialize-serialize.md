---
locale: fr
source_revision: b7aac8fb7a35df9ddf10adc50d88fb1bfd0169290aa22d5f4ad96531651b49c8
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

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
