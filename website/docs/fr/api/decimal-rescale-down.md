---
locale: fr
source_revision: e228552c41e4946b118a075a0bdaa811f6ff511347af55baa0908ccb2c361609
translation_provenance: en-US-french-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

Modifier le nombre de décimales en tronquant vers zéro.

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize/number entier ; renvoie Decimal, ou InvalidPrecision si la précision est négative. Down signifie vers zéro, pas vers moins l’infini. Augmenter la précision ajoute des zéros. L’entrée reste inchangée et le résultat est initialisé.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
