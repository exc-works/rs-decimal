---
locale: fr
source_revision: be3c0e366eb3a51881c8080b2a8de1d5f2faa06f8be431e48c0342e9c8d4c0e5
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

Arrondir au plus proche, vers zéro à mi-chemin.

```rust
HalfDown = 4
```

## Paramètres, résultat et utilisation courante

Discriminant entier 4 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
