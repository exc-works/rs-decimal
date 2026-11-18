---
locale: de
source_revision: 8a30b5196c371952fcc2508159879f3619eaf7085b839003d31c6721b7f9cf86
translation_provenance: en-US-german-reviewed
api_id: "validation"
---
# validation

Regelfehler in Reihenfolge sammeln und übersetzen.

```rust
pub mod validation
```

## Parameter, Rückgabe und übliche Verwendung

Rust benötigt das im Browserbuild aktivierte Feature validation. Das Modul vereint exakte Regelauswertung und eigene Nachrichtenvorlagen ohne Framework oder globale Registry. Verwenden Sie im Browser den validation-Namensraum. Feldnamen sind Text des Aufrufers und werden bei Übersetzung wörtlich eingesetzt. Verwenden Sie required explizit, wenn fehlende Eingaben scheitern müssen.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
