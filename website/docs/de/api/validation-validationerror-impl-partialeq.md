---
locale: de
source_revision: 88d2cfb170b637934d794950f5a609aa140f9cabce333b9eba44daa4d52bc8fc
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationError::impl PartialEq"
---
# validation::ValidationError::impl PartialEq

Die Rust-Implementierung von PartialEq verwenden.

```rust
impl PartialEq for validation::ValidationError
```

## Parameter, Rückgabe und übliche Verwendung

ValidationError-Gleichheit vergleicht field, tag und ursprünglichen parameter gemeinsam, nicht nur übersetzte Meldungen. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationerror-impl-partialeq" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
