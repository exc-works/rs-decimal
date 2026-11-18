---
locale: fr
source_revision: 91bb241ee917c58402441fd00f901b8aa75a693fa5e9137596c74ca66e946c7e
translation_provenance: en-US-french-reviewed
api_id: Decimal::int64
---
# Decimal::int64

Convertir en entier de largeur fixe avec un indicateur de réussite.

```rust
fn int64 (& self) -> (i64 , bool)
```

## Paramètres, résultat et utilisation courante

Sans paramètre ; renvoie (i64, bool), représenté par [bigint, boolean]. Les valeurs entières dans la plage renvoient l’entier et true. Les fractions et dépassements renvoient (0, false), pas une valeur tronquée ni une exception. L’échelle seule ne détermine pas si une valeur est entière : 12.00 réussit. Vérifiez toujours l’indicateur pour distinguer un échec d’un zéro valide.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="conversion-decimal-int64" locale="fr" />

## Limites du navigateur

Le transport des entiers ne passe pas par des number JavaScript avec perte. Les tuples de conversion conservent l’ordre Rust valeur/indicateur ; false ne garantit pas un résultat exact. Les entrées empruntées restent inchangées et les constructeurs initialisent les résultats. La [politique de ressources](../resource-policy) limite les entrées entières, l’échelle de sortie et la croissance des coefficients rationnels, séparément des erreurs de bibliothèque. La syntaxe d’import Rust est explicitement indiquée comme testée nativement.

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
