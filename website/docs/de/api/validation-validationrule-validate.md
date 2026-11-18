---
locale: de
source_revision: 92b27b2e44b2095f2d604aead4c4f01fb922094a0083fe1c3c5e2509eaa4b3d7
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

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
