---
locale: fr
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
translation_provenance: en-US-french-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

Importer les modes d’arrondi depuis la racine du crate.

```rust
pub use rounding::RoundingMode as RoundingMode
```

## Paramètres, résultat et utilisation courante

Utilisez rs_decimal::RoundingMode. Il s’agit de la même énumération, pas d’une seconde implémentation. Le navigateur expose l’objet nommé RoundingMode sans syntaxe d’import Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
