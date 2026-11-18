---
locale: de
source_revision: e175d4d7de2a038dff837a099025c66d8337b4480eacf9bc8d2de1f1d5950025
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule::parse"
---
# validation::ValidationRule::parse

Eine exakte Dezimalregel auswerten.

```rust
fn parse (tag : & str , parameter : & str) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

tag: &str/string, parameter: &str/string; liefert `Result<ValidationRule, DecimalError>`/ein Regelhandle. Verwenden Sie vollständige Tags wie decimal_gt. Vergleichsparameter sind Dezimaltext; between nimmt lower~upper, max_precision ein nichtnegatives isize. Unbekannte Tags, fehlerhafte Parameter und umgekehrte Bereiche liefern InvalidArgument. Auf wasm32 ist isize vorzeichenbehaftet mit 32 Bits. Parameterlose Regeln erhalten Parametertext, interpretieren ihn aber nicht. Regeln vergleichen exakte Decimal-Werte ohne Gleitkommakonvertierung. decimal_required prüft Initialisierung; initialisierte Null besteht daher. eq/ne/gt/gte/lt/lte vergleichen auch nicht initialisierte Null numerisch. between verwendet einschließlich geltende lower~upper-Grenzen und verlangt zusätzlich Initialisierung. positive/negative/nonzero verlangen Initialisierung. max_precision prüft gespeicherte Skala einschließlich erhaltener nachgestellter Nullen; required ist dadurch nicht impliziert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-parse" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
