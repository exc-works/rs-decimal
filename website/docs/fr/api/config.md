---
locale: fr
source_revision: 03da42d48c5d8b250daaaa93d59162466d4ca8b88da11e32c7b577b2d36d0747
translation_provenance: en-US-french-reviewed
api_id: "config"
---
# config

Décoder des valeurs de configuration explicitement typées.

```rust
pub mod config
```

## Paramètres, résultat et utilisation courante

L’adaptateur de configuration typé accepte le texte et ses octets, le texte numérique exact, les entiers signés/non signés sur 128 bits, les flottants et les valeurs Decimal/NullDecimal empruntées. Rust exige config. Le décodage non nullable rejette Null et les Nullable invalides au lieu d’inventer zéro. Les Decimal existants et Nullable valides sont clonés avec leur représentation complète, y compris le zéro non initialisé. Text/Number/Bytes analysent du texte décimal exact sans supprimer les guillemets SQL. Les flottants comportent déjà l’arrondi binaire ; utilisez du texte ou bigint pour conserver les chiffres source exacts. Les variantes du navigateur appartiennent à une seule exécution et empruntent les handles/tampons jusqu’au décodage.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="config-serde-config" locale="fr" />

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
