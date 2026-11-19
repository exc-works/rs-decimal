---
locale: fr
source_revision: 23ce03e01f179e79048016168712d7a3101de4dd03467bd8aefb98f6fd47b269
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Default
---
# Decimal::impl Default

Créer un zéro numérique non initialisé avec Rust Default.

```rust
impl Default for Decimal
```

## Utilisation courante et sémantique

La valeur par défaut est numériquement égale à un zéro construit, mais is_nil() vaut true. L’API JavaScript propose Decimal.default() ; cette page décrit le trait Rust lui-même.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-default" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
