---
locale: de
source_revision: 584886d027ae1ffb7977be9193069c421cfe24c70d485c0551ed585a1d8e7407
translation_provenance: en-US-german-reviewed
api_id: Decimal::strip_trailing_zeros
---
# Decimal::strip_trailing_zeros

Überflüssige nachgestellte Nachkommanullen ohne Änderung des Zahlenwerts entfernen.

```rust
fn strip_trailing_zeros (& self) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert einen unabhängigen Decimal. Zehnerfaktoren werden nur bei positiver Skala entfernt; ganzzahlige Nullen wie in 1200 bleiben erhalten. Jede numerische Null wird zur initialisierten Null mit Skala null.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="rounding-decimal-strip-trailing-zeros" locale="de" />

## Verwandte Methoden

[RoundingMode](./roundingmode) · [rescale](./decimal-rescale) · [significant_figures](./decimal-significant-figures)
