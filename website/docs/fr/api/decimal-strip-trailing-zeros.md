---
locale: fr
source_revision: a66a15086c7943a9a541c1451a0fe544a096af2a9b938eb44bd20730eac8975d
translation_provenance: en-US-french-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

Retirer les zéros fractionnaires finaux superflus sans changer le nombre.

```rust
fn strip_trailing_zeros (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un Decimal indépendant. Les facteurs dix ne sont retirés que tant que l’échelle est positive : les zéros entiers, comme dans 1200, ne sont donc pas supprimés. Tout zéro numérique devient un zéro initialisé d’échelle zéro.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
