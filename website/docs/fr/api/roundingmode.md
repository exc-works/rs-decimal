---
locale: fr
source_revision: ac4a10a855b39a59f0636e888eaecef62de05250e4c407dbf6ae0398e83f87d9
translation_provenance: en-US-french-reviewed
api_id: RoundingMode
---
# RoundingMode

Choisir l’une des sept règles d’arrondi explicites.

```rust
pub enum RoundingMode
```

## Paramètres, résultat et utilisation courante

Utilisez les variantes nommées de l’énumération en Rust et les champs de RoundingMode dans le bridge. Les discriminants Rust 0–6 correspondent aux nombres du navigateur. Down va vers zéro ; Up s’éloigne de zéro ; Ceiling va vers plus l’infini. À mi-chemin, HalfUp/HalfDown/HalfEven choisissent respectivement l’éloignement de zéro, le rapprochement de zéro ou le chiffre pair. Unnecessary exige un résultat exact. Aucun réglage global implicite de l’arrondi n’est appliqué.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
