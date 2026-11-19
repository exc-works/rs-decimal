---
locale: de
source_revision: dce0eeed904ef266b7197b18fd411cf158683a2fe3f9074f19a11965111706e9
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::impl PartialEq
---
# RoundingMode::impl PartialEq

Die Rust-PartialEq-Implementierung für Rundungsmodi verwenden.

```rust
impl PartialEq for RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

RoundingMode ist ein kleines Copy-Enum: Kopieren erhält die gewählte Regel. Gleichheit vergleicht Varianten, Debug gibt ihre Namen aus. Dieses Beispiel prüft native Rust-Trait-Syntax; Browsermodi sind unveränderliche Zahlenwerte.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-partialeq" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
