---
locale: de
source_revision: c829b94ceed61e0a65772da94750a53ecb8269151bdb6f1efd6a7c407ad09f12
translation_provenance: en-US-german-reviewed
api_id: "reexport::validation::default_validation_messages"
---
# reexport::validation::default_validation_messages

Eine neue Map mit Gebietsschemavorlagen erhalten.

```rust
pub use validation::messages::default_validation_messages as default_validation_messages
```

## Parameter, Rückgabe und übliche Verwendung

locale: &str/string; liefert `BTreeMap<String,String>`/ein gewöhnliches Objekt mit allen zwölf Regelvorlagen. Unbekannte Gebietsschemata verwenden Englisch. Jeder Aufruf besitzt seine Map; Änderungen beeinflussen weder spätere Aufrufe noch andere ValidationMessages-Objekte. Der Reexport ist der öffentliche Modulimportpfad derselben Funktion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-reexport-validation-default-validation-messages" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
