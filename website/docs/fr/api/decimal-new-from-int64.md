---
locale: fr
source_revision: fae9d1b20487be5ea60841b603ab060b40d5b639bd9efc686e9fd8f0f555bd79
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_int64
---
# Decimal::new_from_int64

Créer un décimal à partir d’un coefficient signé sur 64 bits et d’une précision en décimales. La valeur et l’état suivent le même comportement que `new_with_prec`.

```rust
fn new_from_int64 (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: i64` → `bigint` ; `prec: isize` → `number` entier. Renvoie `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-int64" locale="fr" />

## Conseils d’utilisation

La valeur est un coefficient, pas un montant déjà mis à l’échelle. Une précision négative produit `InvalidPrecision` ; les valeurs hors de la plage i64 sont des erreurs de paramètre de l’API JavaScript.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
