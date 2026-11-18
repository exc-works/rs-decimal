---
locale: fr
source_revision: fc360056fa6a06d0df76e11b3c2a65adb20ebcb5dcc14e8a0ad1eefc1447a90e
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Ord
---
# Decimal::impl Ord

Trier numériquement des décimaux avec Rust Ord.

```rust
impl Ord for Decimal
```

## Utilisation courante et sémantique

Ord::cmp renvoie Ordering, contrairement à Decimal::cmp qui renvoie -1/0/1. Les différences de représentation n’affectent pas l’ordre.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-ord" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
