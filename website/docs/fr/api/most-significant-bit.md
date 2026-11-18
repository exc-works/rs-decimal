---
locale: fr
source_revision: cbc68f86b580dd7b41c6b741193a12b44e7d04cdf771d632d510adf82d84c1ba
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

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
