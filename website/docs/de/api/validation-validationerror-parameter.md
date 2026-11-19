---
locale: de
source_revision: 9d8774ce76acc603ce438d5ebe120fafe780695ef55b74ea55b1fbe183940981
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
parameter: String
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliches eigenes String/string, das den ursprünglichen Regelparameter einschließlich Leerraum exakt erhält. Es wird einmal als {1} eingesetzt, ohne rekursive Vorlageninterpretation. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
