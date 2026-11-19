---
locale: de
source_revision: 11b8aa485f155a658e5960045fe97e0878f07fd6eb371a471f1440f29be38903
translation_provenance: en-US-german-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

Bei gewählter Nachkommastellenzahl Richtung positive Unendlichkeit runden.

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number. Negative Präzision liefert InvalidPrecision. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
