---
locale: fr
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
translation_provenance: en-US-french-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

Retirer les zéros fractionnaires finaux superflus sans changer le nombre.

```rust
fn strip_trailing_zeros (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un Decimal indépendant. Les facteurs dix ne sont retirés que tant que l’échelle est positive : les zéros entiers, comme dans 1200, ne sont donc pas supprimés. Tout zéro numérique devient un zéro initialisé d’échelle zéro.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
