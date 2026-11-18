---
locale: fr
source_revision: 52852655da8fa507f812b593c8e5ba40f3e2342d29c1bda28c254c4a87933fda
translation_provenance: en-US-french-reviewed
api_id: "NullDecimal::impl Serialize"
---
# NullDecimal::impl Serialize

Sérialiser avec le trait Serde de Rust.

```rust
impl Serialize for NullDecimal
```

## Paramètres, résultat et utilisation courante

serializer : S implémentant Serializer ; renvoie S::Ok/S::Error. Le cœur sérialise des chaînes décimales canoniques ou null. Les valeurs nullables invalides sont sérialisées en null ; les valeurs valides dont le zéro interne n’est pas initialisé sont sérialisées en chaîne "0". Les zéros fractionnaires finaux sont supprimés. L’adaptateur serialize() du navigateur choisit serde_json et renvoie le texte JSON complet. Il n’expose pas de sérialiseurs Rust arbitraires.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-nulldecimal-impl-serialize" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
