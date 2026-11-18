---
locale: de
source_revision: 340ee9c355dc0ac1e0c13e6d43aee222d595260389706ec4d9398cbe1a49147c
translation_provenance: en-US-german-reviewed
api_id: reexport::RoundingMode
---
# reexport::RoundingMode

Rundungsmodi aus der Crate-Wurzel importieren.

```rust
pub use rounding::RoundingMode as RoundingMode
```

## Parameter, Rückgabe und übliche Verwendung

Verwenden Sie rs_decimal::RoundingMode. Dies ist dasselbe Enum, keine zweite Implementierung. Der Browser stellt das benannte RoundingMode-Objekt ohne Rust-Importsyntax bereit.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-reexport-roundingmode" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
