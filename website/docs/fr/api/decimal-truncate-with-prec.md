---
locale: fr
source_revision: 6f37682196df5b201aacde1b52aa54d192c469e0d50cfead6f78c24b12704f29
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

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
