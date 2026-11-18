---
locale: fr
source_revision: a19e9628caaaa2c66221e7031ee01f75dd38995c9d2873827f8d6308231f84ca
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

Arrondir au plus proche, avec un dernier chiffre conservé pair à mi-chemin.

```rust
HalfEven = 5
```

## Paramètres, résultat et utilisation courante

Discriminant entier 5 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
