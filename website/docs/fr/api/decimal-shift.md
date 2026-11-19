---
locale: fr
source_revision: 6d142adfffddbbc67008e17859ebe097a896ff7983bb306596fd170c8b867b28
translation_provenance: en-US-french-reviewed
api_id: Decimal::shift
---
# Decimal::shift

Multiplier un décimal par une puissance exacte de dix.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

places : isize/number entier. Une valeur positive déplace le point décimal vers la droite ; une valeur négative, vers la gauche. Cela change la valeur numérique, contrairement à rescale. Le cœur renvoie Overflow lorsqu’un décalage non nul produit une échelle supérieure à i32::MAX. Les contrôles de croissance et d’échelle du navigateur peuvent renvoyer une erreur de ressources distincte avant l’allocation. Même shift(0) initialise le zéro par défaut.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
