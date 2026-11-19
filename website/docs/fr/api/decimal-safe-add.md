---
locale: fr
source_revision: 197565e31e34527cbc33f567c65ff6fb5c5304500454cb2da830919e852c5158
translation_provenance: en-US-french-reviewed
api_id: Decimal::safe_add
---
# Decimal::safe_add

Additionner exactement deux décimaux en exigeant un résultat non négatif.

```rust
fn safe_add (& self , other : & Self) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

other : &Decimal ; renvoie un nouveau Decimal à la plus grande échelle des opérandes. L’alignement est exact et ne supprime aucun chiffre. Rust renvoie Result : seul le résultat doit être non négatif, pas chaque opérande. Un résultat négatif renvoie InvalidArgument ; zéro est accepté.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-safe-add" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
