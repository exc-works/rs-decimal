---
locale: de
source_revision: 22b226c9626e0bfe9b6ad82b4e6efbc955c78f9e4b48acf530ad413ce7ec7c69
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::impl Debug
---
# RoundingMode::impl Debug

Die Rust-Debug-Implementierung für Rundungsmodi verwenden.

```rust
impl Debug for RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

RoundingMode ist ein kleines Copy-Enum: Kopieren erhält die gewählte Regel. Gleichheit vergleicht Varianten, Debug gibt ihre Namen aus. Dieses Beispiel prüft native Rust-Trait-Syntax; Browsermodi sind unveränderliche Zahlenwerte.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-impl-debug" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
