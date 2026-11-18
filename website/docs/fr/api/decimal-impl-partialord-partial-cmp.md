---
locale: fr
source_revision: 1a22225e09dfc3b3b1f7504680c33552c2bd9a622eca096fdf946a5ba9c5cfaa
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl PartialOrd::partial_cmp
---
# Decimal::impl PartialOrd::partial_cmp

Comparer des décimaux avec l’ordre partiel de Rust.

```rust
fn partial_cmp (& self , other : & Self) -> Option < Ordering >
```

## Utilisation courante et sémantique

Toutes les valeurs Decimal sont comparables : partial_cmp renvoie donc Some(Ordering). Le zéro par défaut est comparable à un zéro construit ; des échelles différentes ne produisent pas None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord-partial-cmp" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
