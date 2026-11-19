---
locale: de
source_revision: 50372ca1028f4ab77e1e376da48c589b0fdad5a066b7c436d3b5780901b17b77
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

Nur exakte Ergebnisse zulassen.

```rust
Unnecessary = 6
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 6; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Das Verwerfen von Nullziffern ist exakt; jede verworfene Ziffer ungleich null liefert RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
