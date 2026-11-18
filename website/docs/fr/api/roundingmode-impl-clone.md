---
locale: fr
source_revision: c6bfdc5a275f1528f234a1a54bc09c0ed6112246616c753d2e81d844db37231b
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

Utiliser l’implémentation Rust Clone des modes d’arrondi.

```rust
impl Clone for RoundingMode
```

## Paramètres, résultat et utilisation courante

RoundingMode est une petite énumération Copy : la copie conserve la règle choisie. L’égalité compare les variantes, et Debug affiche leurs noms. Cet exemple vérifie la syntaxe native des traits Rust ; les modes du navigateur sont des valeurs numériques immuables.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
