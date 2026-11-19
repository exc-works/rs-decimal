---
locale: de
source_revision: 8ffc267a7c40a9915ad1544633aa5847696df2f114f2a6275b83514da10320eb
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule::validate"
---
# validation::ValidationRule::validate

Ein Feld gegen eine Regel validieren.

```rust
fn validate (& self , field : & str , value : & Decimal) -> Result < () , ValidationError >
```

## Parameter, Rückgabe und übliche Verwendung

field: &str/string, value: &Decimal/Decimal; liefert `Result<(), ValidationError>`. Erfolg entspricht undefined in JavaScript. Fehlschlag wirft einen JS-ValidationError mit category="validation" und Eigenschaften field/tag/parameter; er unterscheidet sich von DecimalError. Feld und ursprünglicher Parametertext bleiben einschließlich Klammern und Leerraum unverändert. Regel und Wert werden ausgeliehen. validate_decimal sammelt mehrere Fehler ohne Ausnahme.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-validate" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
