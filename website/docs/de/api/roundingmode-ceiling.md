---
locale: de
source_revision: e6982e644ce0db12869f83bce30b6a469943f72b611efb52c20fbf5be010ea23
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::Ceiling
---
# RoundingMode::Ceiling

Richtung positive Unendlichkeit runden.

```rust
Ceiling = 2
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 2; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-ceiling" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
