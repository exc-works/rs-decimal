---
locale: de
source_revision: be26937ee4bcd1a30b10deb30b4ced946f0371e3015f1e537f8c85709084d801
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::impl Clone
---
# RoundingMode::impl Clone

Die Rust-Clone-Implementierung für Rundungsmodi verwenden.

```rust
impl Clone for RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

RoundingMode ist ein kleines Copy-Enum: Kopieren erhält die gewählte Regel. Gleichheit vergleicht Varianten, Debug gibt ihre Namen aus. Dieses Beispiel prüft native Rust-Trait-Syntax; Browsermodi sind unveränderliche Zahlenwerte.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-clone" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
