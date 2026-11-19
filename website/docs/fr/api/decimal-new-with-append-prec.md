---
locale: fr
source_revision: 5a350c30c073c772c4921158a8a0d829d4480ae375ec35f7532a213f3f45e557
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_with_append_prec
---
# Decimal::new_with_append_prec

Conserver la valeur numérique d’un entier signé en stockant davantage de décimales. Cela permet notamment de représenter un montant entier sous la forme 12.00.

```rust
fn new_with_append_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: i64` → `bigint` ; `prec: isize` → `number` entier. Le coefficient stocké devient `value × 10^prec` ; l’échelle devient `prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-append-prec" locale="fr" />

## Conseils d’utilisation

`string()` affiche 12 ; `string_with_trailing_zeros()` affiche 12.00. Contrairement à `new_with_prec`, la valeur numérique ne devient pas 0.12. Une précision négative produit `InvalidPrecision` ; les demandes trop grandes sont rejetées avant l’allocation des puissances de dix.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
