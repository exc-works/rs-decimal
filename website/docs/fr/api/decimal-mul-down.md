---
locale: fr
source_revision: 8f612ca2b723227e5991f08e31b92d45cb4ea058472e86db2c02e8a1c4cc372e
translation_provenance: en-US-french-reviewed
api_id: Decimal::mul_down
---
# Decimal::mul_down

Multiplier des décimaux avec une précision de sortie définie.

```rust
fn mul_down (& self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal. L’échelle du résultat est la plus grande des échelles des opérandes, pas leur somme. Utilisez mul_exact pour conserver tous les chiffres du produit. Down tronque vers zéro, y compris pour les résultats négatifs.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-mul-down" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
