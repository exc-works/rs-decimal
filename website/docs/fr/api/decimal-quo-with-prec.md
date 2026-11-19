---
locale: fr
source_revision: eb9bcc338f948ddd5ec14bfd00bb5bb38695bb5e5eba9b781fe855d89569b0a0
translation_provenance: en-US-french-reviewed
api_id: Decimal::quo_with_prec
---
# Decimal::quo_with_prec

Diviser des décimaux avec une précision explicite ou dérivée des entrées.

```rust
fn quo_with_prec (& self , other : & Self , prec : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; Rust renvoie Result<Decimal, DecimalError>. prec choisit un nombre non négatif de décimales ; une précision négative renvoie InvalidPrecision avant la vérification du diviseur. Un diviseur nul renvoie DivideByZero. mode contrôle l’arrondi ; Unnecessary renvoie RoundUnnecessary pour un résultat inexact.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-quo-with-prec" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
