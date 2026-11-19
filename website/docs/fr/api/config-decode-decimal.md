---
locale: fr
source_revision: 0268b5969d6d0344c17f1c1c3dee925ce05b68c8ea0fc3782a3dde3b820e544c
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

## Méthodes associées

[decode_decimal](./config-decode-decimal) · [decode_nullable](./config-decode-nullable) · [deserialize_json_decimal](./config-deserialize-json-decimal)
