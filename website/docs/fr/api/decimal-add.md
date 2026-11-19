---
locale: fr
source_revision: 1edc6ba7e191babd98a0717864d8696548b2086ee97162b567ea1d847a357dad
translation_provenance: en-US-french-reviewed
api_id: Decimal::add
---
# Decimal::add

Additionner exactement deux décimaux.

```rust
fn add (& self , other : & Self) -> Self
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
