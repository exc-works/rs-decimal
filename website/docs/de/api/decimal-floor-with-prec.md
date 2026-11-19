---
locale: de
source_revision: 8fb2de8eaa72e1390b09fd949ca444ffb681b4ab0d4c0a3535431123d0666727
translation_provenance: en-US-german-reviewed
api_id: Decimal::floor_with_prec
---
# Decimal::floor_with_prec

Bei gewählter Nachkommastellenzahl Richtung negative Unendlichkeit runden.

```rust
fn floor_with_prec (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number; Rust liefert Result<Decimal, DecimalError>, die Bridge Decimal oder eine Ausnahme. Negative Präzision liefert InvalidPrecision. Bei negativen Werten unterscheiden sich Abschneiden und floor: -1.5 wird auf -1 abgeschnitten, aber mit floor zu -2. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
