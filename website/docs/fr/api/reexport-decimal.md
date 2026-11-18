---
locale: fr
source_revision: 893ae85fdbe2ce4fb0a97cdee41fd9675ed135240e7df9b22b449b50279fb13c
translation_provenance: en-US-french-reviewed
api_id: reexport::Decimal
---
# reexport::Decimal

Importer Decimal depuis la racine du crate rs_decimal.

```rust
pub use decimal::Decimal as Decimal
```

## Utilisation courante et sémantique

Utilisez rs_decimal::Decimal ; c’est le chemin d’import public du même élément, pas une implémentation numérique supplémentaire. JavaScript ne possède pas la syntaxe use de Rust ; consultez la référence du bridge correspondante.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-reexport-decimal" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
