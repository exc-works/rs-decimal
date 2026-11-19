---
locale: de
source_revision: 521743c60e4285630a1e202d92aebd0d0b9f4764a73c56a223f9ec0c5662a6d6
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
pub struct ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

Ein Fehlerdatensatz mit öffentlichen Eigenschaften field, tag und parameter. ValidationRule.validate liefert ihn bei Fehlschlag; validate_decimal liefert eine Liste. JavaScript stellt bearbeitbare Felddatensätze für die Übersetzung bereit. Sie sind weder DecimalError-Kategorien noch Ausnahmemeldungen. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
