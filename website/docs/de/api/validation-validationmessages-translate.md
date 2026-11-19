---
locale: de
source_revision: d21e2ccb9059c66b32b65cffe1a9b447ab9a7fc0664cf5fa020b0d334fe294fd
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationMessages::translate"
---
# validation::ValidationMessages::translate

Validierungsfehler mit eigenen Vorlagen übersetzen.

```rust
fn translate (& self , errors : & [ValidationError]) -> Vec < String >
```

## Parameter, Rückgabe und übliche Verwendung

errors: &[ValidationError]/Array; liefert ein Zeichenfolgenarray in derselben Reihenfolge. Gebietsschemaabgleich ignoriert Groß-/Kleinschreibung und akzeptiert Bindestriche/Unterstriche sowie Sprach-/Regionssuffixe; zh-Hant/zh-TW wählen traditionelles Chinesisch. en-US/en-GB verwenden Englisch; unbekannte Gebietsschemata fallen auf Englisch zurück. {0} fügt das Feld und {1} den ursprünglichen Parameter in einem Durchgang ein; eingefügte Klammern werden nicht erneut expandiert. Ergebnisse sind Klartext; stellen Sie sie als Text dar, nicht als HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-translate" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
