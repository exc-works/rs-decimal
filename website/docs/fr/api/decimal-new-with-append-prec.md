---
locale: fr
source_revision: 7e14f36bf17f5e06d930d4aefd01137eff4589a325eb7eee0237e89f396a3ba8
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

## Remarques et limites

`string()` affiche 12 ; `string_with_trailing_zeros()` affiche 12.00. Contrairement à `new_with_prec`, la valeur numérique ne devient pas 0.12. Une précision négative produit `InvalidPrecision` ; les demandes trop grandes sont rejetées avant l’allocation des puissances de dix.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
