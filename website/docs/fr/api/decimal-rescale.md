---
locale: fr
source_revision: 0deb49519f3f009c2dba973109de20fc0fcdfe90b9617749c506123c4c387997
translation_provenance: en-US-french-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

Modifier le nombre de décimales avec un mode d’arrondi explicite.

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec compte les décimales, pas les chiffres significatifs ; mode est l’une des sept valeurs de RoundingMode. Augmenter la précision ajoute des zéros. Réduire la précision n’arrondit que les chiffres non nuls supprimés. Une précision négative renvoie InvalidPrecision ; Unnecessary renvoie RoundUnnecessary si un arrondi est nécessaire. Les résultats initialisent le zéro par défaut ; les entrées empruntées restent inchangées.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
