---
locale: de
source_revision: 9d0e7a5134f00bf353febb8beb1a65cb4dacb67d9c7e5b41dbb343b7baa674b0
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.field"
---
# validation::ValidationError.field

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
field: String
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliches eigenes String/string mit dem vom Aufrufer angegebenen Feldnamen. Dies ist weder ein Eigenschaftspfad noch ein HTML-Fragment. Klammern werden bei der Übersetzung wörtlich eingefügt. Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-field" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
