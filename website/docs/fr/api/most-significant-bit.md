---
locale: fr
source_revision: e0008f3df5627ec04d695568de0e59f79f0a5617252c018a2798ad7b622bab4d
translation_provenance: en-US-french-reviewed
api_id: most_significant_bit
---
# most_significant_bit

Trouver l’indice du bit de poids fort à 1, en comptant depuis zéro.

```rust
fn most_significant_bit (value : & BigInt) -> Result < usize , DecimalError >
```

## Paramètres, résultat et utilisation courante

value : &BigInt, représenté par bigint. Renvoie Result<usize, DecimalError>/number. Zéro et un renvoient tous deux 0 ; 120 renvoie 6. Une entrée négative renvoie InvalidArgument. C’est une fonction libre, pas une méthode Decimal ; fournissez coefficient() si c’est l’entier recherché.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-most-significant-bit" locale="fr" />

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
