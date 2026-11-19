---
locale: fr
source_revision: b834c8dbe54adc1326f4715bae0b5a7e4a551bc4817e951238d3a76cb1960aa5
translation_provenance: en-US-french-reviewed
api_id: Decimal::safe_sub
---
# Decimal::safe_sub

Soustraire exactement des décimaux en exigeant un résultat non négatif.

```rust
fn safe_sub (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre. Rust renvoie Result : seul le résultat doit être non négatif, pas chaque opérande. Un résultat négatif renvoie InvalidArgument ; zéro est accepté.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-sub" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
