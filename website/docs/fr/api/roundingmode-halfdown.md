---
locale: fr
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

Arrondir au plus proche, vers zéro à mi-chemin.

```rust
HalfDown = 4
```

## Paramètres, résultat et utilisation courante

Discriminant entier 4 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
