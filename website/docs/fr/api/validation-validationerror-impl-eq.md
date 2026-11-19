---
locale: fr
source_revision: cfe29d83921c9d9fef3ddeed3842336ccc12799d1b0ed231fac999e4d87de7f7
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError::impl Eq"
---
# validation::ValidationError::impl Eq

Utiliser l’implémentation Rust Eq.

```rust
impl Eq for validation::ValidationError
```

## Paramètres, résultat et utilisation courante

L’égalité de ValidationError compare ensemble field, tag et le paramètre d’origine ; elle ne compare pas seulement les messages traduits. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-eq" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
