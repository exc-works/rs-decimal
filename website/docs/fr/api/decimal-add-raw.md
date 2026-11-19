---
locale: fr
source_revision: c983f6ed089705857a773a0bcfbea0bf32771eb2c76c31e24f0160ce317c8980
translation_provenance: en-US-french-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

Modifier le coefficient non mis à l’échelle par un entier i64.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Paramètres, résultat et utilisation courante

value : i64, représenté par bigint dans le bridge. L’échelle est conservée : à l’échelle 2, ajouter ou soustraire 1 modifie le nombre de 0.01, pas de 1. Rust renvoie un nouveau Decimal. Le bridge rejette les number et les bigint hors de la plage i64.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
