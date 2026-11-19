---
locale: de
source_revision: db21c0833b97369db1efcc9a73b34501c5dcac820064e9658bbb1cb161d8dec2
translation_provenance: en-US-german-reviewed
api_id: "validation::default_validation_messages"
---
# validation::default_validation_messages

Eine neue Map mit Gebietsschemavorlagen erhalten.

```rust
fn default_validation_messages (locale : & str) -> BTreeMap < String , String >
```

## Parameter, Rückgabe und übliche Verwendung

locale: &str/string; liefert `BTreeMap<String,String>`/ein gewöhnliches Objekt mit allen zwölf Regelvorlagen. Unbekannte Gebietsschemata verwenden Englisch. Jeder Aufruf besitzt seine Map; Änderungen beeinflussen weder spätere Aufrufe noch andere ValidationMessages-Objekte. Der Reexport ist der öffentliche Modulimportpfad derselben Funktion.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-default-validation-messages" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
