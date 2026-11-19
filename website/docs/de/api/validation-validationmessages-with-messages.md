---
locale: de
source_revision: 71b55f4d77a06034a780155a4f624e277f9df950e286fd15014d7cc9e03ee581
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationMessages::with_messages"
---
# validation::ValidationMessages::with_messages

Englische Validierungsvorlagen überschreiben.

```rust
fn with_messages (messages : BTreeMap < String , String >) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

messages: `BTreeMap<String,String>`/ein Objekt mit Zeichenfolgenwerten; liefert `Result<ValidationMessages, DecimalError>`/ein Nachrichtenhandle. Überschreibt nur angegebene Schlüssel; alle anderen Regeln behalten englische Standardwerte. Erlaubt sind die zwölf decimal_*-Regeln. Leere Vorlagen, unbekannte Schlüssel, unpassende Klammern und andere Platzhalter als {0}/{1} liefern InvalidArgument. Wiederholte gültige Platzhalter sind erlaubt. Globale oder zuvor konstruierte Vorlagen bleiben unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationmessages-with-messages" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
