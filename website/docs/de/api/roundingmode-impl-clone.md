---
locale: de
source_revision: c6bfdc5a275f1528f234a1a54bc09c0ed6112246616c753d2e81d844db37231b
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

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
