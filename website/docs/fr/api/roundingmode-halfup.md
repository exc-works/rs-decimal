---
locale: fr
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

Arrondir au plus proche, en s’éloignant de zéro à mi-chemin.

```rust
HalfUp = 3
```

## Paramètres, résultat et utilisation courante

Discriminant entier 3 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
