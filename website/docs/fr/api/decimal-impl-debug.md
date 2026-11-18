---
locale: fr
source_revision: 150ae4511881de6cce3960793291dd735b00fc7aa5175f86d5957a93c70d0c13
translation_provenance: en-US-french-reviewed
api_id: Decimal::impl Debug
---
# Decimal::impl Debug

Inspecter un décimal lors du débogage Rust.

```rust
impl Debug for Decimal
```

## Utilisation courante et sémantique

La représentation de diagnostic dérivée inclut les champs internes ; ce n’est pas un format de sérialisation stable. La sortie du navigateur utilise des instantanés explicites.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-decimal-impl-debug" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
