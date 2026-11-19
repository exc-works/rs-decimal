---
locale: fr
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

Inspecter l’enregistrement d’échec de validation.

```rust
field: String
```

## Paramètres, résultat et utilisation courante

String/string publique possédée contenant le nom de champ fourni par l’appelant. Ce n’est ni un chemin d’accès à une propriété ni un fragment HTML. Les accolades sont insérées littéralement lors de la traduction. Modifier un enregistrement d’erreur ne change ni la règle ni le décimal.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
