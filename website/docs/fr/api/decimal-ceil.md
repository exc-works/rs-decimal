---
locale: fr
source_revision: 53cafbe90f3ab017866bcf2afb140d94d6b1a293f7aa6ca2a32a775110c8f82c
translation_provenance: en-US-french-reviewed
api_id: Decimal::ceil
---
# Decimal::ceil

Arrondir à un entier vers plus l’infini.

```rust
fn ceil (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un nouveau Decimal d’échelle zéro. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
