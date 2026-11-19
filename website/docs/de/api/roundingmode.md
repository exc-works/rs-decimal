---
locale: de
source_revision: ac4a10a855b39a59f0636e888eaecef62de05250e4c407dbf6ae0398e83f87d9
translation_provenance: en-US-german-reviewed
api_id: RoundingMode
---
# RoundingMode

Eine von sieben expliziten Rundungsregeln auswählen.

```rust
pub enum RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

Verwenden Sie benannte Enumvarianten in Rust und RoundingMode-Felder in der Bridge. Rust-Diskriminanten 0–6 entsprechen den Browserzahlen. Down bedeutet Richtung null, Up von null weg, Ceiling Richtung positive Unendlichkeit. HalfUp/HalfDown/HalfEven entscheiden exakte Gleichstände von null weg/Richtung null/zur geraden Ziffer. Unnecessary verlangt Exaktheit. Es gibt keine implizite globale Rundungseinstellung.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
