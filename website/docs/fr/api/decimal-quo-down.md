---
locale: fr
source_revision: 24cee5102dc473cb8fed8902e9829dd352863649ca89859b6c19bb3f001aa710
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo_down
---
# Decimal::quo_down

Diviser des décimaux avec une précision explicite ou dérivée des entrées.

```rust
fn quo_down (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. L’échelle de sortie est la plus grande des échelles des opérandes. Deux entrées entières produisent donc un entier : 1 / 3 donne 0. Utilisez quo_with_prec pour une sortie fractionnaire. Un diviseur nul renvoie DivideByZero. Down tronque vers zéro, pas vers moins l’infini.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-down" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
