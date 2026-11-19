---
locale: fr
source_revision: d6f697a3d962962b5c45eb345607fbc6629bac0f2d7f0f5c403da5649a7d1d4a
translation_provenance: en-US-french-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Arrondir au plus proche, au pair à mi-chemin, avec un nombre choisi de décimales.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## Paramètre

`prec: isize` — Nombre entier de décimales à conserver.

## Valeur de retour

Un Decimal initialisé, même pour un zéro par défaut. La valeur est empruntée. Augmenter la précision ajoute des zéros.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). Utilisez [`rescale`](./decimal-rescale) pour choisir un autre mode d’arrondi.

## Erreurs

Une précision négative renvoie `InvalidPrecision`. L’API JavaScript lève l’erreur correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
