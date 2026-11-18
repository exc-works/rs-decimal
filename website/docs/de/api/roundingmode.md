---
locale: de
source_revision: 24579cc5f98ad85ad1558995873e9949e2e4e6ae6e01cc4702b2ee7ca18a14ea
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

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
