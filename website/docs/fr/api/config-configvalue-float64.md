---
locale: fr
source_revision: 21a2321ebb07dad0e89ac689f0921883a544f6575bbd19e235e51c9aee9deb73
translation_provenance: en-US-french-reviewed
api_id: "config::ConfigValue::Float64"
---
# config::ConfigValue::Float64

Sélectionner une entrée de configuration Float64.

```rust
Float64 (f64)
```

## Paramètres, résultat et utilisation courante

Transporte un binary64 via la conversion décimale la plus courte du cœur. Les valeurs non finies renvoient Unmarshal ; les chiffres déjà perdus par l’arrondi flottant ne peuvent pas être récupérés.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-configvalue-float64" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
