---
locale: de
source_revision: 80ca29e7b285c6bbe14fbaae400143c560d7d0375268641d09bff74efd458b6d
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::HalfDown
---
# RoundingMode::HalfDown

Zum nächsten Wert runden, bei Gleichstand Richtung null.

```rust
HalfDown = 4
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 4; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfdown" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
