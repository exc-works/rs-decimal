---
locale: de
source_revision: b1efdab89860dbce37ea848eed30ac657af5e945bd5643b2378d7d893a2ce9a2
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::from_raw
---
# RoundingMode::from_raw

Einen ganzzahligen Rundungsmoduscode validieren.

```rust
fn from_raw (value : i32) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

value: i32/ganzzahlige number. Liefert für 0–6 den Modus, für andere i32-Werte InvalidArgument. Gebrochene, nicht endliche oder außerhalb von i32 liegende Browserwerte scheitern vor Rust an der Parameterprüfung. Die JavaScript-API ruft den Rust-Validator auf und liefert den numerischen Modus.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-from-raw" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
