---
locale: fr
source_revision: 94e5c200ef4c48b600cdc1b46dd606d6ca8e9e2741aedbb2bac50f53ac53727e
translation_provenance: en-US-french-reviewed
api_id: Decimal::new
---
# Decimal::new

Créer un entier initialisé. Ce constructeur convient à un entier signé sur 64 bits, par exemple un décompte ou un montant entier.

```rust
fn new (value : i64) -> Self
```

## Paramètres et résultat

`value: i64` → `bigint` JavaScript. Le résultat a une échelle nulle.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="constructor-new" locale="fr" />

## Conseils d’utilisation

Utilisez `12n`, pas un number JavaScript. L’API JavaScript rejette les valeurs hors de la plage i64 ; utilisez `new_from_string` pour les fractions décimales.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
