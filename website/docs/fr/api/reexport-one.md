---
locale: fr
source_revision: 29e91f626fd685fb0b5987fc104c1ba0e2752897194773305e3d64dcc1b16f1a
translation_provenance: en-US-french-reviewed
api_id: reexport::one
---
# reexport::one

Importer one depuis la racine du crate rs_decimal.

```rust
pub use decimal::one as one
```

## Utilisation courante et sémantique

Utilisez rs_decimal::one ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-one" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
