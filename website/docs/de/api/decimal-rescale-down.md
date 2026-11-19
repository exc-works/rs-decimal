---
locale: de
source_revision: e9dbb721c8987afdaedb4f74b013156db52924dc6bfa528e95669ba4396942ff
translation_provenance: en-US-german-reviewed
api_id: Decimal::rescale_down
---
# Decimal::rescale_down

Nachkommastellen durch Abschneiden in Richtung null ändern.

```rust
fn rescale_down (& self , prec : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec: isize/ganzzahlige number; liefert Decimal oder bei negativer Präzision InvalidPrecision. Down bedeutet Richtung null, nicht negative Unendlichkeit. Höhere Präzision hängt Nullen an. Die Eingabe bleibt unverändert, das Ergebnis ist initialisiert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale-down" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
