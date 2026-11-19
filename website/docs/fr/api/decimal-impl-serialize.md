---
locale: fr
source_revision: fd8204204097a2ca3064943a4cf9e6b865ae769705e2693bc4a79f71be2a4582
translation_provenance: en-US-french-reviewed
api_id: "Decimal::impl Serialize"
---
# Decimal::impl Serialize

Sérialiser avec le trait Serde de Rust.

```rust
impl Serialize for Decimal
```

## Paramètres, résultat et utilisation courante

serializer : S implémentant Serializer ; renvoie S::Ok/S::Error. Le cœur sérialise des chaînes décimales canoniques ou null. Un Decimal non initialisé est sérialisé en null ; un zéro initialisé est sérialisé en chaîne "0". Les zéros fractionnaires finaux sont supprimés. L’adaptateur serialize() du navigateur choisit serde_json et renvoie le texte JSON complet. Il n’expose pas de sérialiseurs Rust arbitraires.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-decimal-impl-serialize" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
