---
locale: de
source_revision: e0028e1081b0309d3901334cb20b25aa450fe2c46a4c364db5668c9213b4477b
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::Up
---
# RoundingMode::Up

Von null weg runden.

```rust
Up = 1
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 1; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-up" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
