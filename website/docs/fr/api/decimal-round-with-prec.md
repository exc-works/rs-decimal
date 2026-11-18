---
locale: fr
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
translation_provenance: en-US-french-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Arrondir au plus proche, au pair à mi-chemin, avec un nombre choisi de décimales.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## Paramètre

`prec: isize` — Nombre de décimales. Le bridge accepte un nombre JavaScript entier.

## Valeur de retour

Un Decimal initialisé, même pour un zéro par défaut. La valeur est empruntée. Augmenter la précision ajoute des zéros.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). Utilisez [`rescale`](./decimal-rescale) pour choisir un autre mode d’arrondi.

## Erreurs

Une précision négative renvoie `InvalidPrecision`. Le bridge lève l’erreur correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
