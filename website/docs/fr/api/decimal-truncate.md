---
locale: fr
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
translation_provenance: en-US-french-reviewed
api_id: Decimal::truncate
---
# Decimal::truncate

Arrondir à un entier vers zéro.

```rust
fn truncate (& self) -> Self
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie un nouveau Decimal d’échelle zéro. Pour les valeurs négatives, la troncature diffère de floor : -1.5 est tronqué à -1, mais floor donne -2. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
