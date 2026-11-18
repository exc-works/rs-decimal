---
locale: de
source_revision: 4973633c1cdc3ddfb696d250ca871028b9835fa4187520f393245e4690f6a0c3
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

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
