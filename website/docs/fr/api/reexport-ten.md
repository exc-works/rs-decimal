---
locale: fr
source_revision: 76d9cb7e766229b955c894271a5c238caac503dc592850e7b5cbe4a6a3eb33d3
translation_provenance: en-US-french-reviewed
api_id: reexport::ten
---
# reexport::ten

Importer ten depuis la racine du crate rs_decimal.

```rust
pub use decimal::ten as ten
```

## Utilisation courante et sémantique

Utilisez rs_decimal::ten ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-ten" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
