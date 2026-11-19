---
locale: fr
source_revision: db004727284a336c6f0e549dbc589384cfceb8780f95c6036a8917d02c730891
translation_provenance: en-US-french-reviewed
api_id: Decimal::floor
---
# Decimal::floor

Arrondir à un entier vers moins l’infini.

```rust
fn floor (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un nouveau Decimal d’échelle zéro. Pour les valeurs négatives, la troncature diffère de floor : -1.5 est tronqué à -1, mais floor donne -2. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
