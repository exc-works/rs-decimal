---
locale: fr
source_revision: 262ffddf12b7585e3d9a91337def5d4995343306864bd291c3fadc2686ec5701
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError::impl Clone"
---
# validation::ValidationError::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for validation::ValidationError
```

## Paramètres, résultat et utilisation courante

Clone copie la règle complète, la table de modèles ou les champs d’échec. La copie est possédée indépendamment. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-clone" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
