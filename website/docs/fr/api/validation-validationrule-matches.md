---
locale: fr
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

Évaluer une règle décimale exacte.

```rust
fn matches (& self , value : & Decimal) -> bool
```

## Paramètres, résultat et utilisation courante

value : &Decimal/Decimal ; renvoie bool/boolean et préserve la règle comme la valeur. Les règles comparent exactement les valeurs Decimal sans conversion flottante. decimal_required vérifie l’initialisation : un zéro initialisé passe donc le test. eq/ne/gt/gte/lt/lte comparent numériquement, même pour un zéro non initialisé. between utilise lower~upper avec bornes incluses et exige en plus l’initialisation. positive/negative/nonzero exigent l’initialisation. max_precision vérifie l’échelle stockée, y compris les zéros finaux conservés ; cela n’implique pas required.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
