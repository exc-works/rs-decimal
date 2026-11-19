---
locale: fr
source_revision: d588c3aa025efc49b40f1c2c9f6f76c0ff1df249830b32d566fb6a6a5794cdc5
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::impl Eq
---
# RoundingMode::impl Eq

Utiliser l’implémentation Rust Eq des modes d’arrondi.

```rust
impl Eq for RoundingMode
```

## Paramètres, résultat et utilisation courante

RoundingMode est une petite énumération Copy : la copie conserve la règle choisie. L’égalité compare les variantes, et Debug affiche leurs noms. Cet exemple vérifie la syntaxe native des traits Rust ; les modes du navigateur sont des valeurs numériques immuables.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-eq" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
