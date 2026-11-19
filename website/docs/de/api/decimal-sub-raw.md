---
locale: de
source_revision: 3fdc16376a6b5df85a2ce7cc24f6688fcdf41771e793df75d13c0d1b90a448b1
translation_provenance: en-US-german-reviewed
api_id: Decimal::sub_raw
---
# Decimal::sub_raw

Den unskalierten Koeffizienten um eine i64-Ganzzahl ändern.

```rust
fn sub_raw (& self , value : i64) -> Self
```

## Parameter, Rückgabe und übliche Verwendung

value: i64, in JavaScript als bigint dargestellt. Die Skala bleibt erhalten: Bei Skala 2 ändert Addieren/Subtrahieren von 1 die Zahl um 0.01, nicht um 1. Rust liefert einen neuen Decimal. number und bigint außerhalb von i64 weist die JavaScript-API ab.

<script setup>
import MethodExample from "../../_components/MethodExample.vue";
</script>

<MethodExample id="arithmetic-decimal-sub-raw" locale="de" />

## Verwandte Methoden

[add](./decimal-add) · [mul_exact](./decimal-mul-exact) · [quo_with_prec](./decimal-quo-with-prec) · [RoundingMode](./roundingmode)
