---
locale: fr
source_revision: 2558a21d5d46dd2598e1fc940985c24d9012b531617d6e30f108a6afe7d17531
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

## Remarques et limites

La valeur est un coefficient, pas un montant déjà mis à l’échelle. Une précision négative produit `InvalidPrecision` ; les valeurs hors de la plage i64 sont des erreurs de paramètre du bridge.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
