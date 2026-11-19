---
locale: fr
source_revision: 73057fa1830878a540ff385b7411b32617a1d65647f7c283e5495a0cce3aca84
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
