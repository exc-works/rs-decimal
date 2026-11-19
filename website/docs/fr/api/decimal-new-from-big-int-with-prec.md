---
locale: fr
source_revision: 89b834d2c11952f4f6bb0f4503b5e891bf052db7e48a72abe6c367e79b9b8c68
translation_provenance: en-US-french-reviewed
api_id: Decimal::new_from_big_int_with_prec
---
# Decimal::new_from_big_int_with_prec

Copier un coefficient de taille arbitraire et définir son nombre de décimales. Les grands entiers d’entrée restent exacts.

```rust
fn new_from_big_int_with_prec (value : & BigInt , prec : isize) -> Result < Self , DecimalError >
```

## Paramètres et résultat

`value: &BigInt` → `bigint` ; `prec: isize` → `number` entier. Renvoie `value × 10^-prec` en empruntant l’entrée Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new-from-big-int-with-prec" locale="fr" />

## Conseils d’utilisation

Cela change l’interprétation du coefficient, pas ses chiffres. Une précision négative produit `InvalidPrecision`.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
