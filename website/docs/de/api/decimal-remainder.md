---
locale: de
source_revision: 64d94a480e185cbb38c975ef60712d5ae4ac373961962b675392db6ef9375f09
translation_provenance: en-US-german-reviewed
api_id: Decimal::remainder
---
# Decimal::remainder

Ganzzahlteil und unskalierten Nachkommarest trennen.

```rust
fn remainder (& self) -> (BigInt , BigInt)
```

## Parameter, Rückgabe und übliche Verwendung

Keine Parameter; liefert (BigInt, BigInt), abgebildet auf [bigint, bigint]. Der erste Wert wird Richtung null abgeschnitten; der zweite ist ein vorzeichenbehafteter Koeffizientenrest, kein Decimal-Bruchteil. Für -12.750 entsteht (-12, -750): Der zweite Wert benötigt die ursprüngliche Skala 3, um -0.750 zu bedeuten. Die Standardnull liefert (0, 0).

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="owned-decimal-remainder" locale="de" />

## Verwandte Methoden

[add_owned](./decimal-add-owned) · [quo_rem](./decimal-quo-rem) · [remainder](./decimal-remainder) · [power](./decimal-power)
