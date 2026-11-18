---
locale: de
source_revision: 027ed2edff10b36d0b034b15dae2ca9de5412e5b203ccede0d881df699befe4b
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule::impl Clone"
---
# validation::ValidationRule::impl Clone

Die Rust-Implementierung von Clone verwenden.

```rust
impl Clone for validation::ValidationRule
```

## Parameter, Rückgabe und übliche Verwendung

Clone kopiert die vollständige Regel, Vorlagenmap oder Fehlerfelder. Die Kopie besitzt ihre Daten unabhängig. Dies ist native Rust-Trait-Syntax, keine Browsermethode.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-impl-clone" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
