---
locale: fr
source_revision: dfbccd4ef90aecedf2d24565d11016e8068251bd103af1f82ea5d3b25f25cee0
translation_provenance: en-US-french-reviewed
api_id: Decimal::round
---
# Decimal::round

Arrondir à l’entier le plus proche, au pair à mi-chemin.

```rust
fn round (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un nouveau Decimal d’échelle zéro. Le mode par défaut est ici l’arrondi au pair, dont le traitement des valeurs à mi-chemin diffère de JavaScript Math.round. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
