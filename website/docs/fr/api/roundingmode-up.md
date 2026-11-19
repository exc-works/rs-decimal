---
locale: fr
source_revision: e0028e1081b0309d3901334cb20b25aa450fe2c46a4c364db5668c9213b4477b
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::Up
---
# RoundingMode::Up

Arrondir en s’éloignant de zéro.

```rust
Up = 1
```

## Paramètres, résultat et utilisation courante

Discriminant entier 1 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-up" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
