---
locale: de
source_revision: e9709f45197c4a553d9eef583ec3ace47a8ab16090d52890478f7fffb689b911
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

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
