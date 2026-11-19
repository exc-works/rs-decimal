---
locale: de
source_revision: 9937fd6977e8a466e16b6a35082706b1183e93c9c873b5eb8e6c66d5ab917b4f
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationMessages"
---
# validation::ValidationMessages

Validierungsfehler mit eigenen Vorlagen übersetzen.

```rust
pub struct ValidationMessages
```

## Parameter, Rückgabe und übliche Verwendung

Besitzt Vorlagen zur Übersetzung von ValidationError-Datensätzen. Verwenden Sie new(locale) oder with_messages(map); Änderungen an einer anderen Vorlagenmap beeinflussen dieses Handle nicht. Gebietsschemaabgleich ignoriert Groß-/Kleinschreibung und akzeptiert Bindestriche/Unterstriche sowie Sprach-/Regionssuffixe; zh-Hant/zh-TW wählen traditionelles Chinesisch. en-US/en-GB verwenden Englisch; unbekannte Gebietsschemata fallen auf Englisch zurück. {0} fügt das Feld und {1} den ursprünglichen Parameter in einem Durchgang ein; eingefügte Klammern werden nicht erneut expandiert. Ergebnisse sind Klartext; stellen Sie sie als Text dar, nicht als HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
