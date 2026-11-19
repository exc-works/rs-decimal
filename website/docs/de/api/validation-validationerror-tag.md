---
locale: de
source_revision: 0b44060089288355d3eba23edfcc95ed4174aef13953ff7dc6b74df01f566b11
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
tag: & 'static str
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliche statische Rust-Zeichenfolge zur Kennzeichnung der fehlgeschlagenen Regel. Der Browser verwendet eine eigene Zeichenfolgeneigenschaft; unbekannte Tags verwenden bei Übersetzung die Kernstandardvorlage "{0} is invalid". Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
