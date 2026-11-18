---
locale: fr
source_revision: bae692e3c5c12038fd1ef26d04bf1880cf624ae83c8c2ec0ad9f2f071f1b9f3a
translation_provenance: en-US-french-reviewed
api_id: Decimal::significant_figures
---
# Decimal::significant_figures

Arrondir à un nombre maximal choisi de chiffres décimaux significatifs.

```rust
fn significant_figures (& self , figures : isize , mode : RoundingMode ,) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

figures : isize positif/number entier ; mode choisit l’arrondi. Contrairement à la précision en décimales, les chiffres significatifs comptent les chiffres du coefficient. Cette méthode n’augmente jamais la précision et peut arrondir les chiffres entiers (1234.5 → 1230 avec trois chiffres). Une valeur figures non positive renvoie InvalidArgument ; un arrondi nécessaire avec Unnecessary renvoie RoundUnnecessary. Une éventuelle retenue du coefficient est d’abord vérifiée par rapport au budget du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-significant-figures" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
