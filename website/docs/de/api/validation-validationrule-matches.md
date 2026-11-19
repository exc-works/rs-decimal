---
locale: de
source_revision: 816a5e8449654efb901339ad5516b3fdb94be2af25e3c709a61823154b83be91
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

## Verwandte Methoden

[parse](./validation-validationrule-parse) · [validate_decimal](./validation-validate-decimal) · [translate](./validation-validationmessages-translate)
