---
locale: fr
source_revision: 572356860c801a3a257544cc180e1dca7bec96afecfd793ab3dbbcce432242dd
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationMessages::impl Clone"
---
# validation::ValidationMessages::impl Clone

Utiliser l’implémentation Rust Clone.

```rust
impl Clone for validation::ValidationMessages
```

## Paramètres, résultat et utilisation courante

Clone copie la règle complète, la table de modèles ou les champs d’échec. La copie est possédée indépendamment. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-impl-clone" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
