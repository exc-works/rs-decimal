---
locale: fr
source_revision: ce0e6eba90e03ef0b3ea77c71d25c9216e58acb759f414a585091bbcd0fa1215
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::impl Copy
---
# RoundingMode::impl Copy

Utiliser l’implémentation Rust Copy des modes d’arrondi.

```rust
impl Copy for RoundingMode
```

## Paramètres, résultat et utilisation courante

RoundingMode est une petite énumération Copy : la copie conserve la règle choisie. L’égalité compare les variantes, et Debug affiche leurs noms. Cet exemple vérifie la syntaxe native des traits Rust ; les modes du navigateur sont des valeurs numériques immuables.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-copy" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
