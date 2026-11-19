---
locale: fr
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

N’accepter que les résultats exacts.

```rust
Unnecessary = 6
```

## Paramètres, résultat et utilisation courante

Discriminant entier 6 ; utilisez la valeur nommée plutôt qu’un nombre arbitraire. La règle s’applique aux deux signes. Supprimer des chiffres nuls reste exact ; supprimer un chiffre non nul renvoie RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
