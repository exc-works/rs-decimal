---
locale: fr
source_revision: bf618b8d345d150746683bfe2d04e5a166f48e4917e60db9f1d6a2719994d91a
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

## Remarques et limites

Cela change l’interprétation du coefficient, pas ses chiffres. Une précision négative produit `InvalidPrecision`. Le playground limite séparément les chiffres d’entrée et l’échelle ; les erreurs `resource` ne signalent pas une perte de précision numérique.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
