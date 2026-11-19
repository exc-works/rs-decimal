---
locale: fr
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
translation_provenance: en-US-french-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

Modifier le coefficient non mis à l’échelle par un entier i64.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Paramètres, résultat et utilisation courante

value : i64, représenté par bigint en JavaScript. L’échelle est conservée : à l’échelle 2, ajouter ou soustraire 1 modifie le nombre de 0.01, pas de 1. Rust renvoie un nouveau Decimal. L’API JavaScript rejette les number et les bigint hors de la plage i64.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
