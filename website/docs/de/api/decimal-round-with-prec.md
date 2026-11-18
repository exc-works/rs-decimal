---
locale: de
source_revision: 9cb35d36232b48dfa841cf9ea23e2fcf6d9b36b6ef45c2f68538158901813e1b
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

## Grenzen im Browser

Ganzzahlparameter müssen in den wasm32-Bereich passen. Der Browser wendet zusätzlich die [Ressourcenrichtlinie](../resource-policy) für Zielskala, Koeffizientenwachstum und Aufrufzahl an. Ressourcenfehler bleiben von numerischen Bibliotheksfehlern getrennt. Ausleihende Methoden konsumieren keine Eingaben; reine Rust-Trait-Syntax ist ausdrücklich als nativ getestet gekennzeichnet.

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
