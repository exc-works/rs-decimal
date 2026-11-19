---
locale: fr
source_revision: b1efdab89860dbce37ea848eed30ac657af5e945bd5643b2378d7d893a2ce9a2
translation_provenance: en-US-french-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Valider un code entier de mode d’arrondi.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : i32/number entier. Renvoie le mode pour 0–6 ; les autres valeurs i32 renvoient InvalidArgument. Dans le navigateur, les valeurs fractionnaires, non finies ou hors de la plage i32 échouent à la validation des paramètres avant Rust. L’API JavaScript appelle le validateur Rust et renvoie le mode numérique.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="fr" />

## Méthodes associées

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
