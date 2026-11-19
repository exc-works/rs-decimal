---
locale: fr
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

Inspecter l’enregistrement d’échec de validation.

```rust
pub struct ValidationError
```

## Paramètres, résultat et utilisation courante

Enregistrement d’échec avec les propriétés publiques field, tag et parameter. ValidationRule.validate le renvoie en cas d’échec ; validate_decimal renvoie une liste. JavaScript expose des enregistrements modifiables adaptés à la traduction. Ce ne sont ni des catégories DecimalError ni des messages d’exception. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
