---
locale: fr
source_revision: 7a4da4f8ddb22857191bcc95e49dde5161555ab6aaf013e72ff12ef5cff5913d
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

Cela change l’interprétation du coefficient, pas ses chiffres. Une précision négative produit `InvalidPrecision`. Le playground limite séparément les chiffres d’entrée et l’échelle ; les erreurs `resource` ne signalent pas une perte de précision numérique.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
