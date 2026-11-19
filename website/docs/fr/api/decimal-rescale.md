---
locale: fr
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
