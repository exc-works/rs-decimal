---
locale: fr
source_revision: 0d0d53ee31abc5089791404b98708465e5ae1271ee54c709858dd761b9a5ca22
translation_provenance: en-US-french-reviewed
api_id: reexport::min
---
# reexport::min

Importer min depuis la racine du crate rs_decimal.

```rust
pub use decimal::min as min
```

## Utilisation courante et sémantique

Utilisez rs_decimal::min ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-min" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
