---
locale: fr
source_revision: e9defed137146374a74cf0b0a14760988cf4cd8ad97babd7fd11a73ccc7037b3
translation_provenance: en-US-french-reviewed
api_id: Decimal::has_fraction
---
# Decimal::has_fraction

Examiner la partie fractionnaire numérique.

```rust
fn has_fraction (& self) -> bool
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie bool/boolean. Le test porte sur la valeur numérique, pas sur une échelle égale à zéro : 12.00 est un entier. Le zéro par défaut est aussi un entier sans partie fractionnaire. Ces requêtes n’initialisent ni ne modifient l’entrée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-has-fraction" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
