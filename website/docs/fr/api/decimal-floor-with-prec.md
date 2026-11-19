---
locale: fr
source_revision: 6ba2cb4a20d5541fd7637c273e7892b620d22f9a54a85ae9d00d1d5387296c75
translation_provenance: en-US-french-reviewed
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

Arrondir vers moins l’infini avec un nombre choisi de décimales.

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize/number entier. Une précision négative produit InvalidPrecision. Pour les valeurs négatives, la troncature diffère de floor : -1.5 est tronqué à -1, mais floor donne -2. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
