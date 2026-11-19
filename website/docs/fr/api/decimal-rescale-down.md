---
locale: fr
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
