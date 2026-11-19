---
locale: fr
source_revision: 2cba3a645b36c53756ae013b43e89956dee5ee0f736cbeccecbeb94e8966c7b4
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

## Conseils d’utilisation

La précision compte les décimales, pas les chiffres significatifs. Cette méthode n’ajoute pas des zéros à un entier dont la valeur resterait inchangée ; utilisez `new_with_append_prec` pour obtenir 12.00. Une précision négative produit `InvalidPrecision`. Un coefficient nul conserve l’échelle demandée.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
