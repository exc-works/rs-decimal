---
locale: fr
source_revision: 0da87c25bd0ca11269721d2b8d815d7bac612f73d7110e87ce497372ebbaf01e
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError::impl Debug"
---
# validation::ValidationError::impl Debug

Utiliser l’implémentation Rust Debug.

```rust
impl Debug for validation::ValidationError
```

## Paramètres, résultat et utilisation courante

Debug produit un texte de diagnostic, pas un format sérialisé stable de validation. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-debug" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
