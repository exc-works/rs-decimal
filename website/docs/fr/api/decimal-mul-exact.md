---
locale: fr
source_revision: 74fe800675809309f5a6802d09df41ce125ec348c7dc823b0a9a763856b0bb52
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul_exact
---
# Decimal::mul_exact

Multiplier exactement des décimaux.

```rust
fn mul_exact (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est la somme des échelles des opérandes ; tous les chiffres du produit sont conservés. Un dépassement de l’échelle dans le cœur renvoie Overflow. Il n’y a pas de paramètre de mode d’arrondi.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-exact" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
