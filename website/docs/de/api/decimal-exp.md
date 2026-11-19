---
locale: de
source_revision: f662d4a18feb3987ba22a3e426a493d661feab0dd71696533c5ec29aefece86d
translation_provenance: en-US-german-reviewed
api_id: Decimal::exp
---
# Decimal::exp

Die Exponentialfunktion e^x berechnen. Keine Parameter. Die Standardausgabepräzision ist max(Eingabenachkommastellen, 20), nicht 20 signifikante Ziffern. Der Kern erlaubt alle reellen Eingaben. exp(0) ist initialisierte eins; ein ausreichend negatives Argument kann bei angeforderter Präzision korrekt auf initialisierte Null runden. Der Kern verwendet einschließende Intervalle und akzeptiert ein Ergebnis nur bei zertifizierter Rundung. Er rundet HalfEven ohne Modusparameter; gelingt die Zertifizierung innerhalb der Kernbudgets nicht, folgt NoConvergence statt einer ungeprüften Näherung. Eingaben bleiben unverändert, erfolgreiche Ergebnisse sind initialisiert.

```rust
fn exp (& self) -> Result < Self , DecimalError >
```

## Übliche Verwendung

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="log-exp-decimal-exp" locale="de" />

## Verwandte Methoden

[ln](./decimal-ln) · [log2](./decimal-log2) · [log10](./decimal-log10) · [exp](./decimal-exp)
