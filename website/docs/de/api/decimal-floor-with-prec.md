---
locale: de
source_revision: 6ba2cb4a20d5541fd7637c273e7892b620d22f9a54a85ae9d00d1d5387296c75
translation_provenance: en-US-german-reviewed
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

Bei gewählter Nachkommastellenzahl Richtung negative Unendlichkeit runden.

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number. Negative Präzision liefert InvalidPrecision. Bei negativen Werten unterscheiden sich Abschneiden und floor: -1.5 wird auf -1 abgeschnitten, aber mit floor zu -2. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
