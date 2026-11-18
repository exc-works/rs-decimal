---
locale: fr
source_revision: 78d4bcb73439eeefbcaa373b59f6572230d9fd809bde6e3821566e584e60e547
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_with_prec
---
# Decimal::new_with_prec

Interpréter un entier signé comme un coefficient avec une précision en décimales. Par exemple, 12 avec une précision de 2 représente 0.12.

```rust
fn new_with_prec (value : i64 , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: i64` → `bigint` ; `prec: isize` → `number` entier. Renvoie `value × 10^-prec`.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-with-prec" locale="fr" />

## Remarques et limites

La précision compte les décimales, pas les chiffres significatifs. Cette méthode n’ajoute pas des zéros à un entier dont la valeur resterait inchangée ; utilisez `new_with_append_prec` pour obtenir 12.00. Une précision négative produit `InvalidPrecision`. Un coefficient nul conserve l’échelle demandée.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
