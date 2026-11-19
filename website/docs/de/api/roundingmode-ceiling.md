---
locale: de
source_revision: 8de86b9c337d17cfc9fdcc766a8c16ae7e2cfcd7e721947844d22305f2992bfa
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

Richtung positive Unendlichkeit runden.

```rust
Ceiling = 2
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 2; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
