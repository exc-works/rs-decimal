---
locale: de
source_revision: b1d5b92283e0de37c46d70318c8e896d60bc04df8806e365913a4c12caa7a8d9
translation_provenance: en-US-german-reviewed
api_id: Decimal::ceil
---
# Decimal::ceil

Richtung positive Unendlichkeit zu einer Ganzzahl runden.

```rust
fn ceil (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert einen neuen Decimal mit Skala null. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-ceil" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
