---
locale: fr
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

Évaluer une règle décimale exacte.

```rust
pub struct ValidationRule
```

## Paramètres, résultat et utilisation courante

Créez une règle avec parse(tag, parameter), puis réutilisez-la avec matches ou validate. Les paramètres sont vérifiés lors de la construction ; les handles du navigateur appartiennent à une exécution et peuvent être libérés explicitement. Les règles comparent exactement les valeurs Decimal sans conversion flottante. decimal_required vérifie l’initialisation : un zéro initialisé passe donc le test. eq/ne/gt/gte/lt/lte comparent numériquement, même pour un zéro non initialisé. between utilise lower~upper avec bornes incluses et exige en plus l’initialisation. positive/negative/nonzero exigent l’initialisation. max_precision vérifie l’échelle stockée, y compris les zéros finaux conservés ; cela n’implique pas required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
