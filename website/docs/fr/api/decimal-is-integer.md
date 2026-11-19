---
locale: fr
source_revision: e73578c50b0181c9f243a7f97c0c8dbdcdf0b57587c7ac98a36dc6043a2ce666
translation_provenance: en-US-french-reviewed
api_id: Decimal::is_integer
---
# Decimal::is_integer

Vérifier que la valeur numérique n’a pas de partie fractionnaire.

```rust
fn is_integer (& self) -> bool
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie bool/boolean. Le test porte sur la valeur numérique, pas sur une échelle égale à zéro : 12.00 est un entier. Le zéro par défaut est aussi un entier sans partie fractionnaire. Ces requêtes n’initialisent ni ne modifient l’entrée.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-is-integer" locale="fr" />

## Méthodes associées

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
