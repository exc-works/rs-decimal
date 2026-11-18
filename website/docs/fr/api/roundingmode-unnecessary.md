---
locale: fr
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
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

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
