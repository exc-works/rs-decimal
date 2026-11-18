---
locale: fr
source_revision: 71bc56362fd9f1c25a96932ba1ffd795db889517f58a951726bcd78e58fe2bc7
translation_provenance: en-US-french-reviewed
api_id: NullDecimal::impl Default
---
# NullDecimal::impl Default

Créer une valeur nullable absente avec Rust Default.

```rust
impl Default for NullDecimal
```

## Utilisation courante et sémantique

La validité et l’initialisation interne valent toutes deux false. Le bridge fournit NullDecimal.default().

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="type-nulldecimal-impl-default" locale="fr" />

Cet exemple de syntaxe Rust est exécuté dans les tests natifs et ne prétend pas prendre en charge les traits dans le navigateur.

## Documentation associée

[Decimal](./decimal) · [NullDecimal](./nulldecimal) · [new_from_string](./decimal-new-from-string)
