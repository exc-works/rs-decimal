---
locale: de
source_revision: bb4dd0948b4d4be09c12115b5c13abd8421926284cdbcfed281913fc4636aeb5
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

Zum nächsten Wert runden, bei Gleichstand von null weg.

```rust
HalfUp = 3
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 3; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
