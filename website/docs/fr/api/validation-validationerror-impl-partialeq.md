---
locale: fr
source_revision: 7f1c2832917badced6e78a9c287520234452b297109e8406a028c3703581c859
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Utiliser l’implémentation Rust PartialEq.

```rust
impl PartialEq for validation::ValidationError
```

## Paramètres, résultat et utilisation courante

L’égalité de ValidationError compare ensemble field, tag et le paramètre d’origine ; elle ne compare pas seulement les messages traduits. C’est une syntaxe de trait Rust natif, pas une méthode du navigateur.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
