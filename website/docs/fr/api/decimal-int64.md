---
locale: fr
source_revision: f83d4eab7c44d20bd9f7f112876d8de08078fd38cb0bbaaf09e60fe2891d5d1e
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

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
