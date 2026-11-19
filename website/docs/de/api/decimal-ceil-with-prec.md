---
locale: de
source_revision: 2d051b010383c0adad95eaeacec22cce35ee5a02d1fddebba48aa1d5a1f4d4ae
translation_provenance: en-US-german-reviewed
api_id: Decimal::ceil_with_prec
---
# Decimal::ceil_with_prec

Bei gewählter Nachkommastellenzahl Richtung positive Unendlichkeit runden.

```rust
fn ceil_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number; Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine Ausnahme. Negative Präzision liefert InvalidPrecision. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
