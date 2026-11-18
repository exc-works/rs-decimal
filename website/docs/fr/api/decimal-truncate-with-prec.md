---
locale: fr
source_revision: cce5cfd2a6bfd3edbc32da84fa417cbea83f61f7b1bcf9e77c7c5b49aaed3994
translation_provenance: en-US-french-reviewed
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

Arrondir vers zéro avec un nombre choisi de décimales.

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

prec : isize/number entier ; renvoie Result<Decimal, DecimalError> en Rust, et Decimal ou une exception dans le bridge. Une précision négative produit InvalidPrecision. Pour les valeurs négatives, la troncature diffère de floor : -1.5 est tronqué à -1, mais floor donne -2. Les entrées sont empruntées et inchangées. Les résultats sont initialisés, même pour un zéro par défaut en entrée. Augmenter la précision ajoute des zéros sans changer la valeur numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
