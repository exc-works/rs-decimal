---
locale: fr
source_revision: b55dba0922642cda4e225685c47d97a54da01243942c47cb892b8c9136202f1a
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for validation::ValidationRule
```

## Paramètres, résultat et utilisation courante

Clone copie la règle complète, la table de modèles ou les champs d’échec. La copie est possédée indépendamment. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
