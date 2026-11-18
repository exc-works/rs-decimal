---
locale: de
source_revision: 377152e0483ddcbebcd02f161fe043c129efdc1f915e3eb70b399c28e3d3906b
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError"
---
# validation::ValidationError

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
pub struct ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

Ein Fehlerdatensatz mit öffentlichen Eigenschaften field, tag und parameter. ValidationRule.validate liefert ihn bei Fehlschlag; validate_decimal liefert eine Liste. JavaScript stellt bearbeitbare Felddatensätze für die Übersetzung bereit. Sie sind weder DecimalError-Kategorien noch Ausnahmemeldungen. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
