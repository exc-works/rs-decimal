---
locale: fr
source_revision: 8e2ccd4d5de92023ccad23e3b1ce732438fad5dab0ed6652057ad04ade8ca4a5
translation_provenance: en-US-french-reviewed
api_id: reexport::hundred
---
# reexport::hundred

Importer hundred depuis la racine du crate rs_decimal.

```rust
pub use decimal::hundred as hundred
```

## Utilisation courante et sémantique

Utilisez rs_decimal::hundred ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-hundred" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
