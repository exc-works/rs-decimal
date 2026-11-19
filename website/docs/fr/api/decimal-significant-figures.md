---
locale: fr
source_revision: 6155fe36a4ec8e1b03a89c20c5c7247a068af20681f3b463de98b38c447a2097
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
