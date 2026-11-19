---
locale: fr
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

Inspecter l’enregistrement d’échec de validation.

```rust
parameter: String
```

## Paramètres, résultat et utilisation courante

String/string publique possédée conservant exactement le paramètre de règle d’origine, espaces compris. Il est substitué une seule fois à {1}, sans interprétation récursive du modèle. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
