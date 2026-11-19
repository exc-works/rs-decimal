---
locale: fr
source_revision: 50b84c140827ca9ee14623ef754490d47ed0de164e4d5e55011356b86e00545d
translation_provenance: en-US-french-reviewed
api_id: Decimal::shift
---
# Decimal::shift

Multiplier un décimal par une puissance exacte de dix.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

places : isize/number entier. Une valeur positive déplace le point décimal vers la droite ; une valeur négative, vers la gauche. Cela change la valeur numérique, contrairement à rescale. Le cœur renvoie Overflow lorsqu’un décalage non nul produit une échelle supérieure à i32::MAX. Même shift(0) initialise le zéro par défaut.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
