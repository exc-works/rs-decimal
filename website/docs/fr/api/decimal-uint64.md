---
locale: fr
source_revision: c526a43d69654fb21b49f56d858016f45f88e7f7f6ec8eaa0c13c3e5e939cd03
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

## Méthodes associées

[float32](./decimal-float32) · [int64](./decimal-int64) · [big_rat](./decimal-big-rat) · [new_from_big_rat](./decimal-new-from-big-rat)
