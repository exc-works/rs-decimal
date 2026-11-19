---
locale: de
source_revision: d24f0cd3d78b3594804e8def855ad76024ffbd370d84b3931d49427d7730909f
translation_provenance: en-US-german-reviewed
api_id: Decimal::truncate
---
# Decimal::truncate

Richtung null zu einer Ganzzahl runden.

```rust
fn truncate (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert einen neuen Decimal mit Skala null. Bei negativen Werten unterscheiden sich Abschneiden und floor: -1.5 wird auf -1 abgeschnitten, aber mit floor zu -2. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-truncate" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
