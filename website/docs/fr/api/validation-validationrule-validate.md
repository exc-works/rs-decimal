---
locale: fr
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
translation_provenance: en-US-french-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

Valider un champ selon une règle.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## Paramètres, résultat et utilisation courante

field : &str/string, value : &Decimal/Decimal ; renvoie `Result<(), ValidationError>`. Une réussite vaut undefined en JavaScript. Un échec lève une ValidationError JavaScript avec category="validation" et les propriétés field/tag/parameter ; elle est distincte de DecimalError. Le texte du champ et du paramètre d’origine reste inchangé, accolades et espaces compris. La règle et la valeur sont empruntées. Utilisez validate_decimal pour recueillir plusieurs échecs sans lever d’exception.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="fr" />

## Méthodes associées

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
