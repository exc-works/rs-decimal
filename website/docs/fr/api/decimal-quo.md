---
locale: fr
source_revision: 0d6262b2eb7c6955f187e4d63b88d16a39d035f514ca55c738d74b10a8d69203
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo
---
# Decimal::quo

Diviser des décimaux avec une précision explicite ou dérivée des entrées.

```rust
fn quo (& self , other : & Self , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. L’échelle de sortie est la plus grande des échelles des opérandes. Deux entrées entières produisent donc un entier : 1 / 3 donne 0 avec Down, mais 1 avec Up. Utilisez quo_with_prec pour une sortie fractionnaire. Un diviseur nul renvoie DivideByZero. mode contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary pour un résultat inexact.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
