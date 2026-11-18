---
locale: fr
source_revision: 37fcbc194399d8059373fc9c0c61833d997ffb41871435eb1c6867b7870d1acc
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Clone
---
# Decimal::impl Clone

Cloner un décimal en conservant toute sa représentation.

```rust
impl Clone for Decimal
```

## Utilisation courante et sémantique

Clone copie le coefficient, l’échelle et l’état d’initialisation ; JavaScript utilise l’adaptateur nommé clone() plutôt que la syntaxe d’un trait Rust.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-clone" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
