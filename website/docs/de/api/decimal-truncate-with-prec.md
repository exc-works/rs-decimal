---
locale: de
source_revision: d9bfbd2d29924d270b97f6a4251656ca421cb89e51e3bdcdc1ecdf2cdefab10f
translation_provenance: en-US-german-reviewed
api_id: Decimal::truncate_with_prec
---
# Decimal::truncate_with_prec

Bei gewählter Nachkommastellenzahl Richtung null runden.

```rust
fn truncate_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number. Negative Präzision liefert InvalidPrecision. Bei negativen Werten unterscheiden sich Abschneiden und floor: -1.5 wird auf -1 abgeschnitten, aber mit floor zu -2. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
