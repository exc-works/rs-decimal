---
locale: fr
source_revision: db43c880650b6eb59e9c805ec0cdb77a4879108bb98a5d2e10ec38f65ac55f46
translation_provenance: en-US-french-reviewed
api_id: Decimal::sub
---
# Decimal::sub

Soustraire exactement des décimaux.

```rust
fn sub (& self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
