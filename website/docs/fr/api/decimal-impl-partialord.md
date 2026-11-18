---
locale: fr
source_revision: 25b1ec5a4bb0557629fac6d66ca48c015418489e6280f42443d66425e8372440
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl PartialOrd
---
# Decimal::impl PartialOrd

Comparer des décimaux avec l’ordre partiel de Rust.

```rust
impl PartialOrd for Decimal
```

## Utilisation courante et sémantique

Toutes les valeurs Decimal sont comparables : partial_cmp renvoie donc Some(Ordering). Le zéro par défaut est comparable à un zéro construit ; des échelles différentes ne produisent pas None.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-partialord" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
