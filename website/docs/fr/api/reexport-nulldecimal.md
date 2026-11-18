---
locale: fr
source_revision: 05a4d32c73f6c4d59c8e657272c093b1db00da738b0234c960526209d3e3196c
translation_provenance: en-US-french-reviewed
api_id: reexport::NullDecimal
---
# reexport::NullDecimal

Importer NullDecimal depuis la racine du crate rs_decimal.

```rust
pub use decimal::NullDecimal as NullDecimal
```

## Utilisation courante et sémantique

Utilisez rs_decimal::NullDecimal ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-nulldecimal" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
