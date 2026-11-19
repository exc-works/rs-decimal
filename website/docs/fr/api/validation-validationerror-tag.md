---
locale: fr
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

Inspecter l’enregistrement d’échec de validation.

```rust
tag: & 'static str
```

## Paramètres, résultat et utilisation courante

Chaîne statique publique Rust identifiant la règle en échec. Le navigateur utilise une propriété de chaîne possédée ; les étiquettes inconnues utilisent le modèle par défaut du cœur "{0} is invalid" lors de la traduction. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
