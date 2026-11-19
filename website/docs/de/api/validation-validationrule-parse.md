---
locale: de
source_revision: 906bb0bd7555c68ed2da4c7be205d647a1ca94b724e2e70f5d1160d968357278
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

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
