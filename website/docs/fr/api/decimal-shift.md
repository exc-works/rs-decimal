---
locale: fr
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
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

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
