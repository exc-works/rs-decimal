---
locale: fr
source_revision: 0db8fa19358a4f2f8842ab2f8da65e8e108f7914cbdecf8f41f654ca5171b70c
translation_provenance: en-US-french-reviewed
api_id: reexport::max
---
# reexport::max

Importer max depuis la racine du crate rs_decimal.

```rust
pub use decimal::max as max
```

## Utilisation courante et sémantique

Utilisez rs_decimal::max ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-max" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
