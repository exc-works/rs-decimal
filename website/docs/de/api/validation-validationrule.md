---
locale: de
source_revision: 4a04c1296f4a9e4d0c1eac8d1707242f3d7b87d7302eac99dc6a590f2c03208e
translation_provenance: en-US-german-reviewed
api_id: "validation::ValidationRule"
---
# validation::ValidationRule

Eine exakte Dezimalregel auswerten.

```rust
pub struct ValidationRule
```

## Parameter, Rückgabe und übliche Verwendung

Mit parse(tag, parameter) eine Regel erzeugen und für matches oder validate wiederverwenden. Regelparameter werden einmal bei Konstruktion geprüft; Browserhandles gehören zu einem Lauf und können explizit freigegeben werden. Regeln vergleichen exakte Decimal-Werte ohne Gleitkommakonvertierung. decimal_required prüft Initialisierung; initialisierte Null besteht daher. eq/ne/gt/gte/lt/lte vergleichen auch nicht initialisierte Null numerisch. between verwendet einschließlich geltende lower~upper-Grenzen und verlangt zusätzlich Initialisierung. positive/negative/nonzero verlangen Initialisierung. max_precision prüft gespeicherte Skala einschließlich erhaltener nachgestellter Nullen; required ist dadurch nicht impliziert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="validation-validation-validationrule" locale="de" />

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
