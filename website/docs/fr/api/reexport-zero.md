---
locale: fr
source_revision: d9e6c46654b05ae244bd56f7035b17e55d6f30ce2bee3665fad9be992491dd6d
translation_provenance: en-US-french-reviewed
api_id: reexport::zero
---
# reexport::zero

Importer zero depuis la racine du crate rs_decimal.

```rust
pub use decimal::zero as zero
```

## Utilisation courante et sémantique

Utilisez rs_decimal::zero ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
