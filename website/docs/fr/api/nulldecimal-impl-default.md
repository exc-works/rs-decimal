---
locale: fr
source_revision: c654c9726a283aeb3b59ec2a73dcd6f45d404c2677f244d97a60cf2ff73a642b
translation_provenance: en-US-french-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Créer une valeur nullable absente avec Rust Default.

```rust
impl Default for NullDecimal
```

## Utilisation courante et sémantique

La validité et l’initialisation interne valent toutes deux false. L’API JavaScript fournit NullDecimal.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
