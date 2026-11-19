---
locale: de
source_revision: 50b84c140827ca9ee14623ef754490d47ed0de164e4d5e55011356b86e00545d
translation_provenance: en-US-german-reviewed
api_id: Decimal::shift
---
# Decimal::shift

Eine Dezimalzahl mit einer exakten Zehnerpotenz multiplizieren.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

places: isize/ganzzahlige number. Positive places verschieben den Dezimalpunkt nach rechts, negative nach links. Anders als rescale ändert dies den Zahlenwert. Führt eine Verschiebung eines Werts ungleich null zu einer Skala über i32::MAX, liefert der Kern Overflow. Auch shift(0) initialisiert die Standardnull.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
