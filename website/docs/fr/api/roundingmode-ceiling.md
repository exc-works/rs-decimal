---
locale: fr
source_revision: 8de86b9c337d17cfc9fdcc766a8c16ae7e2cfcd7e721947844d22305f2992bfa
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

Arrondir vers plus l’infini.

```rust
Ceiling = 2
```

## Paramètres, résultat et utilisation courante

Discriminant entier 2 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
