---
locale: fr
source_revision: 9abf8678e26851a7c2e2283fba52cbf7f426bb86feeae64e6b8312a6a7254ba7
translation_provenance: en-US-french-reviewed
api_id: "config::decode_decimal"
---
# config::decode_decimal

Décoder des valeurs de configuration explicitement typées.

```rust
fn decode_decimal (value : ConfigValue < '_ >) -> Result < Decimal , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : ConfigValue ; renvoie `Result<Decimal, DecimalError>`/un nouveau handle Decimal. Rust exige config. Le décodage non nullable rejette Null et les Nullable invalides au lieu d’inventer zéro. Les Decimal existants et Nullable valides sont clonés avec leur représentation complète, y compris le zéro non initialisé. Text/Number/Bytes analysent du texte décimal exact sans supprimer les guillemets SQL. Les flottants comportent déjà l’arrondi binaire ; utilisez du texte ou bigint pour conserver les chiffres source exacts. Les variantes du navigateur appartiennent à une seule exécution et empruntent les handles/tampons jusqu’au décodage.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config-decode-decimal" locale="fr" />

## Limites du navigateur

Les fonctionnalités config et serde sont activées. Le texte et les octets source sont limités à 8192 octets UTF-8 ; Rust contrôle le texte décimal et les chaînes JSON décodées avant l’analyse, afin que les échappements ne puissent pas contourner la [politique de ressources](../resource-policy). Les entiers sur 128 bits exigent un bigint dans les limites. Les handles Decimal/NullDecimal empruntés doivent être encore actifs lors du décodage ; la conversion crée des valeurs indépendantes sans consommer les entrées. Le comportement du trait Serde générique est illustré par des adaptateurs JSON concrets.

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
