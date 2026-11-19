---
locale: de
source_revision: ad5bc09e58ba54af3a20899560f38f4b2682294a4c7489043e5331092c1467a0
translation_provenance: en-US-german-reviewed
api_id: Decimal::add_raw
---
# Decimal::add_raw

Den unskalierten Koeffizienten um eine i64-Ganzzahl ändern.

```rust
fn add_raw (& self , value : i64) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

value: i64, in JavaScript als bigint dargestellt. Die Skala bleibt erhalten: Bei Skala 2 ändert Addieren/Subtrahieren von 1 die Zahl um 0.01, nicht um 1. Rust liefert einen neuen Decimal. number und bigint außerhalb von i64 weist die JavaScript-API ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-add-raw" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
