---
locale: fr
source_revision: 099c3533a52ede06adc92af14955d2d2a20dadf66c185c385b9eda728b52b6ba
translation_provenance: en-US-french-reviewed
api_id: reexport::zero
---
# reexport::zero

Importer zero depuis la racine du crate rs_decimal.

```rust
pub use decimal::zero as zero
```

## Utilisation courante et sémantique

Utilisez rs_decimal::zero ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-zero" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
