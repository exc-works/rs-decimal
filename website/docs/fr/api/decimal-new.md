---
locale: fr
source_revision: d578df6b6b71b8ce4b0e165f6e630705dc946bcefab55f8b43fc9cba6d60bb70
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

Utilisez `12n`, pas un number JavaScript. Le bridge rejette les valeurs hors de la plage i64 ; utilisez `new_from_string` pour les fractions décimales.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
