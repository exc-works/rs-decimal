---
locale: de
source_revision: 203acbe634de9e56f24b2f6e3eccc13dcb25f721727e606d7d5a0efdca2a8e02
translation_provenance: en-US-german-reviewed
api_id: Decimal::rescale
---
# Decimal::rescale

Nachkommastellen mit einem expliziten Rundungsmodus ändern.

```rust
fn rescale (& self , prec : isize , mode : RoundingMode) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

prec zählt Nachkommastellen, nicht signifikante Ziffern; mode ist einer der sieben RoundingMode-Werte. Höhere Präzision hängt Nullen an. Bei niedrigerer Präzision beeinflussen nur verworfene Ziffern ungleich null die Rundung. Negative Präzision liefert InvalidPrecision; Unnecessary liefert RoundUnnecessary, wenn Rundung nötig ist. Ergebnisse initialisieren die Standardnull; ausgeliehene Eingaben bleiben unverändert.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-rescale" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
