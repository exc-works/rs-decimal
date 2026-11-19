---
locale: fr
source_revision: 24d7fa4a2221c5f03dbc4893af4fe9b8c3b42496692a1b9ed6c8a03465745caf
translation_provenance: en-US-french-reviewed
api_id: reexport::max
---
# reexport::max

Importer max depuis la racine du crate rs_decimal.

```rust
pub use decimal::max as max
```

## Utilisation courante et sémantique

Utilisez rs_decimal::max ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
