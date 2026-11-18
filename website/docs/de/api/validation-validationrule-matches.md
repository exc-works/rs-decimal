---
locale: de
source_revision: 788e3facaf7bb31d022529aaea08540d91c8a4001537b0085ab4e604ef0e3ae8
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule::matches"
---
# validation::ValidationRule::matches

Eine exakte Dezimalregel auswerten.

```rust
fn matches (& self , value : & Decimal) -> bool
```

## Parameter, Rückgabe und übliche Verwendung

value: &Decimal/Decimal; liefert bool/boolean und erhält Regel und Wert. Regeln vergleichen exakte Decimal-Werte ohne Gleitkommakonvertierung. decimal_required prüft Initialisierung; initialisierte Null besteht daher. eq/ne/gt/gte/lt/lte vergleichen auch nicht initialisierte Null numerisch. between verwendet einschließlich geltende lower~upper-Grenzen und verlangt zusätzlich Initialisierung. positive/negative/nonzero verlangen Initialisierung. max_precision prüft gespeicherte Skala einschließlich erhaltener nachgestellter Nullen; required ist dadurch nicht impliziert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule-matches" locale="de" />

## Grenzen im Browser

Das Feature validation ist aktiviert. Jede Zeichenfolge ist auf 8192 UTF-8-Bytes begrenzt, Regel-/Fehlerlisten und Nachrichtenmaps auf 64 Einträge, JSON-Transport auf 65536 Bytes. Dezimale Regelparameter folgen der [Ressourcenrichtlinie](../resource-policy). Übersetzung prüft vorab konservativ eine Gesamtgrenze von 65536 Bytes für ersetzten Text und kann ein gerade noch passendes Ergebnis ablehnen. Benutzerdefinierte Tags und Felder sind Daten, niemals ausgeführter Code. Validierungsfehler, Konfigurationsfehler und Bridge-Ressourcenfehler bleiben getrennt.

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
