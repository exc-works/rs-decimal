---
locale: fr
source_revision: 823f73234f73cba4c9bfd0b5f1d418fa482625a8365337d1d27eb5d7306b4e6d
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

Utiliser l’implémentation Rust Debug des modes d’arrondi.

```rust
impl Debug for RoundingMode
```

## Paramètres, résultat et utilisation courante

RoundingMode est une petite énumération Copy : la copie conserve la règle choisie. L’égalité compare les variantes, et Debug affiche leurs noms. Cet exemple vérifie la syntaxe native des traits Rust ; les modes du navigateur sont des valeurs numériques immuables.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
