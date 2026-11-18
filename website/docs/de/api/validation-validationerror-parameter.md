---
locale: de
source_revision: 0e561c16634157ba3586cee74da3c90f15491d42428e944428a8ee980c8ddeb4
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.parameter"
---
# validation::ValidationError.parameter

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
parameter: String
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliches eigenes String/string, das den ursprünglichen Regelparameter einschließlich Leerraum exakt erhält. Es wird einmal als {1} eingesetzt, ohne rekursive Vorlageninterpretation. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-parameter" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
