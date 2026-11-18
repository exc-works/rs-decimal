---
locale: de
source_revision: 1d47fa6ae1a12d0a01d3c07d39948787bbb4209bfed6d0edc8d1a4933e938a92
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::Unnecessary
---
# RoundingMode::Unnecessary

Nur exakte Ergebnisse zulassen.

```rust
Unnecessary = 6
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 6; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Das Verwerfen von Nullziffern ist exakt; jede verworfene Ziffer ungleich null liefert RoundUnnecessary.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-unnecessary" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
