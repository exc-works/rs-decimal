---
locale: de
source_revision: 6e97ceecb692f3b30f694df6cf3812a7bc93e427ba0e4c690447e33c151380a9
translation_provenance: en-US-german-reviewed
api_id: Decimal::round
---
# Decimal::round

Zur nächsten Ganzzahl und bei Gleichstand zur geraden Zahl runden.

```rust
fn round (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert einen neuen Decimal mit Skala null. Hier ist HalfEven voreingestellt; dies ist nicht die Gleichstandsregel von JavaScript Math.round. Eingaben werden ausgeliehen und bleiben unverändert. Ergebnisse sind auch bei Standardnull initialisiert. Höhere Präzision hängt Nullen an, ohne den Zahlenwert zu ändern.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
