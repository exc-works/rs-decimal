---
locale: de
source_revision: 6fae0dc88a563ce1dfaf67cbb1ece08366df9aad8969e9320dabf2cf06fa5933
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError.tag"
---
# validation::ValidationError.tag

Den fehlgeschlagenen Validierungsdatensatz untersuchen.

```rust
tag: & 'static str
```

## Parameter, Rückgabe und übliche Verwendung

Öffentliche statische Rust-Zeichenfolge zur Kennzeichnung der fehlgeschlagenen Regel. Der Browser verwendet eine eigene Zeichenfolgeneigenschaft; unbekannte Tags verwenden bei Übersetzung die Kernstandardvorlage "{0} is invalid". Das Bearbeiten eines Fehlerdatensatzes ändert weder Regel noch Dezimalzahl.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-tag" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
