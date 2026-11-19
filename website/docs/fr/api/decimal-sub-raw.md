---
locale: fr
source_revision: 31042d691902e2dc4bdadd92d4688638c315fb08be40c79d11b031b854afc6bc
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

Modifier le coefficient non mis à l’échelle par un entier i64.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## Paramètres, résultat et utilisation courante

value : i64, représenté par bigint dans le bridge. L’échelle est conservée : à l’échelle 2, ajouter ou soustraire 1 modifie le nombre de 0.01, pas de 1. Rust renvoie un nouveau Decimal. Le bridge rejette les number et les bigint hors de la plage i64.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
