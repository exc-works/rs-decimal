---
locale: de
source_revision: 356d7409ac746daa69113ece073c46b80c7b34815cbe6ddfc49fc1c80e686226
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
field: String
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliches eigenes String/string mit dem vom Aufrufer angegebenen Feldnamen. Dies ist weder ein Eigenschaftspfad noch ein HTML-Fragment. Klammern werden bei der Übersetzung wörtlich eingefügt. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
