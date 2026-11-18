---
locale: de
source_revision: 32a89fd803ef33f414f98138648ca1bbfe7cf03c12443bee715ec01ffb382e0d
translation_provenance: en-US-german-reviewed
api_id: Decimal::floor
---
# Decimal::floor

Richtung negative Unendlichkeit zu einer Ganzzahl runden.

```rust
fn floor (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert einen neuen Decimal mit Skala null. Bei negativen Werten unterscheiden sich Abschneiden und floor: -1.5 wird auf -1 abgeschnitten, aber mit floor zu -2. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-floor" locale="de" />

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
