---
locale: de
source_revision: 16147351e8fe026905ac983886e4a94a6ce341eb3c301ee1901cd0719ea82ee1
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Einen ganzzahligen Rundungsmoduscode validieren.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: i32/ganzzahlige number. Liefert für 0–6 den Modus, für andere i32-Werte InvalidArgument. Gebrochene, nicht endliche oder außerhalb von i32 liegende Browserwerte scheitern vor Rust an der Parameterprüfung. Die Bridge ruft den Rust-Validator auf und liefert den numerischen Modus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
