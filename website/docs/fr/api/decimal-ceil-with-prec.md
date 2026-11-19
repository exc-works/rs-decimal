---
locale: fr
source_revision: 2d051b010383c0adad95eaeacec22cce35ee5a02d1fddebba48aa1d5a1f4d4ae
translation_provenance: en-US-french-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

Arrondir vers plus l’infini avec un nombre choisi de décimales.

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize/number entier ; renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception dans le bridge. Une précision négative produit InvalidPrecision. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
