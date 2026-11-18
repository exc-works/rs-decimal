---
locale: fr
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Valider un code entier de mode d’arrondi.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : i32/number entier. Renvoie le mode pour 0–6 ; les autres valeurs i32 renvoient InvalidArgument. Dans le navigateur, les valeurs fractionnaires, non finies ou hors de la plage i32 échouent à la validation des paramètres avant Rust. Le bridge appelle le validateur Rust et renvoie le mode numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="fr" />

## Limites du navigateur

Les paramètres entiers doivent tenir dans la plage wasm32. Le navigateur applique aussi la [politique de ressources](../resource-policy), notamment l’échelle cible, la croissance des coefficients et le nombre d’appels. Les erreurs de ressources restent distinctes des erreurs numériques de la bibliothèque. Les méthodes avec emprunt ne consomment pas les entrées ; la syntaxe des traits propre à Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
