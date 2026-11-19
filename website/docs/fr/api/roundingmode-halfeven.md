---
locale: fr
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

Arrondir au plus proche, avec un dernier chiffre conservé pair à mi-chemin.

```rust
HalfEven = 5
```

## Paramètres, résultat et utilisation courante

Discriminant entier 5 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Down et Up se réfèrent à zéro, pas à moins ou plus l’infini. Les modes Half ne diffèrent qu’à mi-chemin exact. Il n’existe pas de variante Floor distincte ; utilisez floor_with_prec pour cette direction.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
