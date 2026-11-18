---
locale: fr
source_revision: a9093dcd1b9fe39261e7248debd207a7c4986a0a263f3e970359020a9ba293a8
translation_provenance: en-US-french-reviewed
api_id: reexport::between
---
# reexport::between

Importer between depuis la racine du crate rs_decimal.

```rust
pub use decimal::between as between
```

## Utilisation courante et sémantique

Utilisez rs_decimal::between ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-between" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
