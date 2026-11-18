---
locale: de
source_revision: e0f36bf4a4b64a112b7758ba3b58ccfab48769ae252409c4b72638890e0e2a0e
translation_provenance: en-US-german-reviewed
api_id: RoundingMode::HalfUp
---
# RoundingMode::HalfUp

Zum nächsten Wert runden, bei Gleichstand von null weg.

```rust
HalfUp = 3
```

## Parameter, Rückgabe und übliche Verwendung

Ganzzahliger Diskriminant 3; verwenden Sie den benannten Wert statt einer magischen Zahl. Die Regel gilt für beide Vorzeichen. Down und Up beziehen sich auf null, nicht auf negative/positive Unendlichkeit. Half-Modi unterscheiden sich nur bei exakten Mittelwerten. Es gibt keinen separaten Floor-Enumwert; verwenden Sie dafür floor_with_prec.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-roundingmode-halfup" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
