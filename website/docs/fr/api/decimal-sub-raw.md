---
locale: fr
source_revision: 3fdc16376a6b5df85a2ce7cc24f6688fcdf41771e793df75d13c0d1b90a448b1
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

Modifier le coefficient non mis à l’échelle par un entier i64.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## Paramètres, résultat et utilisation courante

value : i64, représenté par bigint en JavaScript. L’échelle est conservée : à l’échelle 2, ajouter ou soustraire 1 modifie le nombre de 0.01, pas de 1. Rust renvoie un nouveau Decimal. L’API JavaScript rejette les number et les bigint hors de la plage i64.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
