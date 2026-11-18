---
locale: fr
source_revision: 08a6704725cb461888907af8e46adcebe3aa92b6c462154ff5ba96f3a955340f
translation_provenance: en-US-french-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Arrondir au plus proche, au pair à mi-chemin, avec un nombre choisi de décimales.

```rust
fn round_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize/number entier ; renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception dans le bridge. Une précision négative produit InvalidPrecision. Le mode par défaut est ici l’arrondi au pair, dont le traitement des valeurs à mi-chemin diffère de JavaScript Math.round. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
