---
locale: fr
source_revision: 317e1f4c19916312bb2823fc0a526f5311afada3dc44c8d3247bdd5fc0f08111
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Ord::cmp
---
# Decimal::impl Ord::cmp

Trier numériquement des décimaux avec Rust Ord.

```rust
fn cmp (& self , other : & Self) -> Ordering
```

## Utilisation courante et sémantique

Ord::cmp renvoie Ordering, contrairement à Decimal::cmp qui renvoie -1/0/1. Les différences de représentation n’affectent pas l’ordre.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord-cmp" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
