---
locale: fr
source_revision: dce0eeed904ef266b7197b18fd411cf158683a2fe3f9074f19a11965111706e9
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

Utiliser l’implémentation Rust PartialEq des modes d’arrondi.

```rust
impl PartialEq for RoundingMode
```

## Paramètres, résultat et utilisation courante

RoundingMode est une petite énumération Copy : la copie conserve la règle choisie. L’égalité compare les variantes, et Debug affiche leurs noms. Cet exemple vérifie la syntaxe native des traits Rust ; les modes du navigateur sont des valeurs numériques immuables.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
