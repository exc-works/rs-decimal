---
locale: fr
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
