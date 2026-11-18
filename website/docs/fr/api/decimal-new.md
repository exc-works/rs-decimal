---
locale: fr
source_revision: fe9121bf755dd82c88fc1274dc43b09d6173d99c7a22441eb84bc6a73587fa71
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

## Remarques et limites

Utilisez `12n`, pas un number JavaScript. Le bridge rejette les valeurs hors de la plage i64 ; utilisez `new_from_string` pour les fractions décimales.

Le navigateur applique également la [politique de ressources](../resource-policy). Les entrées JavaScript sont validées avant l’appel à Rust.

## Méthodes associées

[new_with_prec](./decimal-new-with-prec) · [new_with_append_prec](./decimal-new-with-append-prec) · [new_from_string](./decimal-new-from-string)
