---
locale: fr
source_revision: 309b64e5487e4ac1d2ea8cf88852216653b617cb1ed53a7d6bdd1a0dffa2d494
translation_provenance: en-US-french-reviewed
api_id: Decimal::uint64
---
# Decimal::uint64

Convertir en entier de largeur fixe avec un indicateur de réussite.

```rust
fn uint64 (& self) -> (u64 , bool)
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie (u64, bool), représenté par [bigint, boolean]. Les valeurs entières dans la plage renvoient l’entier et true. Les fractions et dépassements renvoient (0, false), pas une valeur tronquée ni une exception. Les valeurs négatives renvoient également (0, false). L’échelle seule ne détermine pas si une valeur est entière : 12.00 réussit. Vérifiez toujours l’indicateur pour distinguer un échec d’un zéro valide.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-uint64" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
