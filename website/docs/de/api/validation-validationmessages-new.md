---
locale: de
source_revision: 2b24eac398b765662bae8967979a2547332ee566c8d7e9c0bb785ba0d1f9e478
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationMessages::new"
---
# validation::ValidationMessages::new

Validierungsfehler mit eigenen Vorlagen übersetzen.

```rust
fn new (locale : & str) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

locale: &str/string; liefert neue eigene Nachrichtenvorlagen. Gebietsschemaabgleich ignoriert Groß-/Kleinschreibung und akzeptiert Bindestriche/Unterstriche sowie Sprach-/Regionssuffixe; zh-Hant/zh-TW wählen traditionelles Chinesisch. en-US/en-GB verwenden Englisch; unbekannte Gebietsschemata fallen auf Englisch zurück. {0} fügt das Feld und {1} den ursprünglichen Parameter in einem Durchgang ein; eingefügte Klammern werden nicht erneut expandiert. Ergebnisse sind Klartext; stellen Sie sie als Text dar, nicht als HTML.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-new" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
