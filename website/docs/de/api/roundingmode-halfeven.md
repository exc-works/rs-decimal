---
locale: de
source_revision: 84ba3002f85664068a9d9ececc408a7d6f0214a486be55b19a41d101ddf845eb
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::HalfEven
---
# RoundingMode::HalfEven

Zum nächsten Wert runden, bei Gleichstand zur geraden verbleibenden Ziffer.

```rust
HalfEven = 5
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 5; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfeven" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
