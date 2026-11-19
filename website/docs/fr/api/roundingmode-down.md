---
locale: fr
source_revision: be7e92c9ff29d88bdd7b2eadc18fb40911d696405ee369af6496615cdbec068e
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::Down
---
# RoundingMode::Down

Arrondir vers zéro.

```rust
Down = 0
```

## Paramètres, résultat et utilisation courante

Discriminant entier 0 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-down" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
