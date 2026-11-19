---
locale: de
source_revision: ce0e6eba90e03ef0b3ea77c71d25c9216e58acb759f414a585091bbcd0fa1215
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::impl Copy
---
# RoundingMode::impl Copy

Die Rust-Copy-Implementierung für Rundungsmodi verwenden.

```rust
impl Copy for RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

RoundingMode ist ein kleines Copy-Enum: Kopieren erhält die gewählte Regel. Gleichheit vergleicht Varianten, Debug gibt ihre Namen aus. Dieses Beispiel prüft native Rust-Trait-Syntax; Browsermodi sind unveränderliche Zahlenwerte.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-copy" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
