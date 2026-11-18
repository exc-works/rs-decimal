---
locale: de
source_revision: 69f5b453b56bcf2350f3a3cf2e3d29b1035a464f2105cefeff33c4c49c926524
translation_provenance: en-US-german-reviewed
api_id: Decimal::shift
---
# Decimal::shift

Eine Dezimalzahl mit einer exakten Zehnerpotenz multiplizieren.

```rust
fn shift (& self , places : isize) -> Result < Self , DecimalError >
```

## Parameter, Rückgabe und übliche Verwendung

places: isize/ganzzahlige number. Positive places verschieben den Dezimalpunkt nach rechts, negative nach links. Anders als rescale ändert dies den Zahlenwert. Führt eine Verschiebung eines Werts ungleich null zu einer Skala über i32::MAX, liefert der Kern Overflow. Browserprüfungen für Wachstum/Skala können stattdessen vor der Allokation einen separaten Ressourcenfehler liefern. Auch shift(0) initialisiert die Standardnull.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-shift" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
