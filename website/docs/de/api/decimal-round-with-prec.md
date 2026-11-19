---
locale: de
source_revision: 636d6e9ec2d1b0cb7be987deb57dc622bb8084f92601482a3b6418a00ec64c8b
translation_provenance: en-US-german-reviewed
api_id: Decimal::round_with_prec
---
# Decimal::round_with_prec

Bei gewählter Nachkommastellenzahl zum nächsten Wert und bei Gleichstand zur geraden Ziffer runden.

```rust
fn round_with_prec(&self, prec: isize) -> Result<Self, DecimalError>
```

## Parameter

`prec: isize` — Anzahl der Nachkommastellen. Die Bridge erwartet eine ganzzahlige JavaScript-Zahl.

## Rückgabewert

Ein initialisierter Decimal, auch bei einer nicht initialisierten Null als Eingabe. Der Wert wird ausgeliehen. Höhere Präzision hängt Nullen an.

`2.345 → 2.34` · `2.355 → 2.36` (HalfEven). Mit [`rescale`](./decimal-rescale) lässt sich der Rundungsmodus wählen.

## Fehler

Negative Präzision ergibt `InvalidPrecision`. Die Bridge wirft den entsprechenden Fehler.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-round-with-prec" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
